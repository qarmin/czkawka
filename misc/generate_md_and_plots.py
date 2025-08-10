import pandas as pd
import matplotlib.pyplot as plt
import os

# Read the data from the file
df = pd.read_csv('compilation_results.txt', sep='|', engine='python')
df = df.apply(lambda x: x.str.strip() if x.dtype == "object" else x)  # Remove whitespace from headers/values

# Convert columns to appropriate types if needed
df["Output File Size (bytes)"] = pd.to_numeric(df["Output File Size(in bytes)"], errors='coerce')
df["Target Folder Size (bytes)"] = pd.to_numeric(df["Target Folder Size(in bytes)"], errors='coerce')
df["Compilation Time (seconds)"] = pd.to_numeric(df["Compilation Time(seconds)"], errors='coerce')

os.makedirs("charts", exist_ok=True)

plt.figure(figsize=(12, 6))
plt.barh(df["Config"], df["Compilation Time (seconds)"])
plt.xlabel("Compilation Time (seconds)")
plt.title("Compilation Time by Config")
plt.tight_layout()
plt.savefig('charts/complation_time.png')

plt.figure(figsize=(12, 6))
plt.barh(df["Config"], df["Output File Size (bytes)"] / (1024 ** 2))
plt.xlabel("Output File Size (MB)")
plt.title("Output File Size by Config")
plt.tight_layout()
plt.savefig('charts/output_file_size.png')

plt.figure(figsize=(12, 6))
plt.barh(df["Config"], df["Target Folder Size (bytes)"] / (1024 ** 3))
plt.xlabel("Target Folder Size (GB)")
plt.title("Target Folder Size by Config")
plt.tight_layout()
plt.savefig('charts/target_folder_size.png')

