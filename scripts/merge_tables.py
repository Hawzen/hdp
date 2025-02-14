import sys
import math

import pandas as pd

assert len(sys.argv) == 3, "Usage: python merge_tables.py <server.md> <client.md>"

def parse_md(filename):
    """Parse a simple Markdown table file into a DataFrame."""
    with open(filename, encoding='utf-8') as f:
        # Remove blank lines and strip each line
        lines = [line.strip() for line in f if line.strip()]
    # The first line is the header; second line (divider) is skipped.
    headers = [h.strip() for h in lines[0].split('|') if h.strip()]
    data = []
    for line in lines[2:]:
        # Split the row by '|' and remove empty strings.
        row = [cell.strip() for cell in line.split('|') if cell.strip()]
        if row:
            data.append(row)
    return pd.DataFrame(data, columns=headers)

# Parse the Markdown tables
df1 = parse_md(sys.argv[1])
df2 = parse_md(sys.argv[2])

# Add a field 'Received (Server)' to the merged DataFrame and fill it
df1["Received (Server)"] = df1["Source IP (Server)"].map(lambda x: "🫡" if pd.notna(x) else "🤯")

# Convert 'Protocol Number' to integers for proper merging.
df1["Protocol Number"] = df1["Protocol Number"].astype(int)
df2["Protocol Number"] = df2["Protocol Number"].astype(int)

# Merge the DataFrames on 'Protocol Number' using an outer join.
merged = pd.merge(df1, df2, on="Protocol Number", how="outer", suffixes=("_recv", "_send"))

# A bit hacky but whatever
merged["Received (Server)"] = merged["Received (Server)"].fillna("🤯")
merged["Time (μs) (Client)"] = merged["Time (μs) (Client)"].replace("-", 0).replace(math.nan, 0).astype(int)
merged["Time (μs) (Server)"] = merged["Time (μs) (Server)"].replace("-", 0).replace(math.nan, 0).astype(int)
merged["Time difference (μs)"] = merged["Time (μs) (Server)"].astype(int) - merged["Time (μs) (Client)"].astype(int)
merged["Time difference (μs)"] = merged["Time difference (μs)"].map(lambda x: x if x > 0 else math.nan)
merged = merged.drop(columns=["Time (μs) (Server)", "Time (μs) (Client)"])

merged = merged.sort_values("Protocol Number")
# Print the merged DataFrame as a Markdown table.
print(merged.to_markdown(index=False))

