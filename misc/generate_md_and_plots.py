import pandas as pd
import matplotlib.pyplot as plt  # type: ignore[import-not-found]
import matplotlib  # type: ignore[import-not-found]
import os

# Read the data from the file
df = pd.read_csv("compilation_results.txt", sep="|", engine="python")
df = df.apply(lambda x: x.str.strip() if x.dtype == "object" else x)  # Remove whitespace from headers/values

# Convert columns to appropriate types if needed
df["Output File Size (bytes)"] = pd.to_numeric(df["Output File Size(in bytes)"], errors="coerce")
df["Target Folder Size (bytes)"] = pd.to_numeric(df["Target Folder Size(in bytes)"], errors="coerce")
df["Compilation Time (seconds)"] = pd.to_numeric(df["Compilation Time(seconds)"], errors="coerce")
df["Rebuild Time (seconds)"] = pd.to_numeric(df["Rebuild Time(seconds)"], errors="coerce")

matplotlib.rcParams['font.family'] = 'Noto Sans'

os.makedirs("charts", exist_ok=True)

def plot_barh(
        df,
        value_col,
        label_col,
        xlabel,
        title,
        filename,
        fmt="{:.1f}",
        unit_div=1,
        label_fmt=None,
        dropna=False,
        color="C0"
):
    data = df
    if dropna:
        data = data.dropna(subset=[value_col])
    data_sorted = data.sort_values(value_col, ascending=False)
    plt.figure(figsize=(12, 6))
    bars = plt.barh(
        data_sorted[label_col],
        data_sorted[value_col] / unit_div,
        color=color
    )
    plt.xlabel(xlabel)
    plt.title(title)
    plt.tight_layout()
    if label_fmt is None:
        label_fmt = fmt
    plt.bar_label(bars, fmt=label_fmt, color="white", label_type="edge", padding=-50)
    plt.savefig(filename)
    plt.close()


plot_barh(
    df, "Compilation Time (seconds)", "Config",
    "Compilation Time (seconds)", "Compilation Time by Config",
    "charts/compilation_time.png", fmt="{:.1f}s"
)
plot_barh(
    df, "Rebuild Time (seconds)", "Config",
    "Rebuild Time (seconds)", "Rebuild Time by Config",
    "charts/rebuild_time.png", fmt="{:.1f}s"
)
plot_barh(
    df, "Output File Size (bytes)", "Config",
    "Output File Size (MB)", "Output File Size by Config",
    "charts/output_file_size.png", fmt="{:.2f} MB", unit_div=1024**2, dropna=True
)
plot_barh(
    df, "Target Folder Size (bytes)", "Config",
    "Target Folder Size (GB)", "Target Folder Size by Config",
    "charts/target_folder_size.png", fmt="{:.2f} GB", unit_div=1024**3
)

columns = [col for col in df.columns if "(" not in col and "Thread" not in col]
with open("charts/compilation_results.md", "w") as f:
    f.write(df[columns].to_markdown(index=False))
