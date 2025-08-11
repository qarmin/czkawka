import pandas as pd
import matplotlib.pyplot as plt  # type: ignore[import-not-found]
import os

# Read the data from the file
df = pd.read_csv("compilation_results.txt", sep="|", engine="python")
df = df.apply(lambda x: x.str.strip() if x.dtype == "object" else x)  # Remove whitespace from headers/values

# Convert columns to appropriate types if needed
df["Output File Size (bytes)"] = pd.to_numeric(df["Output File Size(in bytes)"], errors="coerce")
df["Target Folder Size (bytes)"] = pd.to_numeric(df["Target Folder Size(in bytes)"], errors="coerce")
df["Compilation Time (seconds)"] = pd.to_numeric(df["Compilation Time(seconds)"], errors="coerce")
df["Rebuild Time (seconds)"] = pd.to_numeric(df["Rebuild Time(seconds)"], errors="coerce")

os.makedirs("charts", exist_ok=True)

# Sort and plot Compilation Time
df_sorted = df.sort_values("Compilation Time (seconds)", ascending=False)
plt.figure(figsize=(12, 6))
plt.barh(df_sorted["Config"], df_sorted["Compilation Time (seconds)"])
plt.xlabel("Compilation Time (seconds)")
plt.title("Compilation Time by Config")
plt.tight_layout()
plt.savefig("charts/compilation_time.png")

df_sorted = df.sort_values("Rebuild Time (seconds)", ascending=False)
plt.figure(figsize=(12, 6))
plt.barh(df_sorted["Config"], df_sorted["Rebuild Time (seconds)"])
plt.xlabel("Rebuild Time (seconds)")
plt.title("Rebuild Time by Config")
plt.tight_layout()
plt.savefig("charts/rebuild_time.png")

# Sort and plot Output File Size
df_filtered = df.dropna(subset=["Output File Size (bytes)"])
df_sorted = df_filtered.sort_values("Output File Size (bytes)", ascending=False)
plt.figure(figsize=(12, 6))
plt.barh(df_sorted["Config"], df_sorted["Output File Size (bytes)"] / (1024**2))
plt.xlabel("Output File Size (MB)")
plt.title("Output File Size by Config")
plt.tight_layout()
plt.savefig("charts/output_file_size.png")

# Sort and plot Target Folder Size
df_sorted = df.sort_values("Target Folder Size (bytes)", ascending=False)
plt.figure(figsize=(12, 6))
plt.barh(df_sorted["Config"], df_sorted["Target Folder Size (bytes)"] / (1024**3))
plt.xlabel("Target Folder Size (GB)")
plt.title("Target Folder Size by Config")
plt.tight_layout()
plt.savefig("charts/target_folder_size.png")

columns = [col for col in df.columns if "(" not in col and "Thread" not in col]
with open("charts/compilation_results.md", "w") as f:
    f.write(df[columns].to_markdown(index=False))
