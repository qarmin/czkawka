import pandas as pd
import matplotlib.pyplot as plt  # type: ignore[import-not-found]
import matplotlib
import os

# Read the data from the file
df = pd.read_csv("compilation_results.txt", sep="|", engine="python")
df = df.apply(lambda x: x.str.strip() if x.dtype == "object" else x)  # Remove whitespace from headers/values

# Convert columns to appropriate types if needed
df["Output File Size (bytes)"] = pd.to_numeric(df["Output File Size(in bytes)"], errors="coerce")
df["Target Folder Size (bytes)"] = pd.to_numeric(df["Target Folder Size(in bytes)"], errors="coerce")
df["Compilation Time (seconds)"] = pd.to_numeric(df["Compilation Time(seconds)"], errors="coerce")
df["Rebuild Time (seconds)"] = pd.to_numeric(df["Rebuild Time(seconds)"], errors="coerce")

matplotlib.rcParams["font.family"] = "Noto Sans"
FONT_SIZE = 13
matplotlib.rcParams.update(
    {
        "font.size": FONT_SIZE,
        "axes.titlesize": FONT_SIZE,
        "axes.labelsize": FONT_SIZE,
        "xtick.labelsize": FONT_SIZE,
        "ytick.labelsize": FONT_SIZE,
        "legend.fontsize": FONT_SIZE,
        "figure.titlesize": FONT_SIZE,
    }
)


os.makedirs("charts", exist_ok=True)


def plot_barh(
    df: pd.DataFrame,
    value_col: str,
    xlabel: str,
    title: str,
    filename: str,
    fmt: str = "{:.1f}",
    unit_div: float = 1,
    label_fmt: str | None = None,
    dropna: bool = False,
    color: str = "C0",
) -> None:
    data = df
    if dropna:
        data = data.dropna(subset=[value_col])
    data_sorted = data.sort_values(value_col, ascending=False)

    plt.figure(figsize=(12, 8))
    bars = plt.barh(data_sorted["Config"], data_sorted[value_col] / unit_div, color=color)

    ax = plt.gca()
    max_val = (data_sorted[value_col] / unit_div).max()
    if max_val is None or max_val == 0:
        x_max = 1
    else:
        x_max = max_val * 1.15
    ax.set_xlim(0, x_max)

    plt.xlabel(xlabel)
    plt.title(title)

    if label_fmt is None:
        label_fmt = fmt
    plt.bar_label(bars, fmt=label_fmt, color="black", padding=5)

    plt.tight_layout()
    plt.savefig(filename)
    plt.close()


plot_barh(
    df,
    "Compilation Time (seconds)",
    "Compilation Time (seconds)",
    "Compilation Time by Config",
    "charts/compilation_time.png",
    fmt="{:.1f}s",
)
plot_barh(
    df,
    "Rebuild Time (seconds)",
    "Rebuild Time (seconds)",
    "Rebuild Time by Config",
    "charts/rebuild_time.png",
    fmt="{:.1f}s",
)
plot_barh(
    df,
    "Output File Size (bytes)",
    "Output File Size (MB)",
    "Output File Size by Config",
    "charts/output_file_size.png",
    fmt="{:.1f} MB",
    unit_div=1024**2,
    dropna=True,
)
plot_barh(
    df,
    "Target Folder Size (bytes)",
    "Target Folder Size (GB)",
    "Target Folder Size by Config",
    "charts/target_folder_size.png",
    fmt="{:.1f} GB",
    unit_div=1024**3,
)

columns = [col for col in df.columns if "(" not in col and "Thread" not in col]
with open("charts/compilation_results.md", "w") as f:
    f.write(df[columns].to_markdown(index=False))
