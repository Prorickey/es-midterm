import pandas as pd

df = pd.read_csv("data/ebd_CA_sancra_relFeb-2026.tsv", sep="\t", low_memory=False)

print(df.head().columns)