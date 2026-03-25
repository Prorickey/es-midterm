import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv("data/ebd_CA_sancra_relFeb-2026.tsv", sep="\t", low_memory=False)

df["OBSERVATION DATE"] = pd.to_datetime(df["OBSERVATION DATE"])
day_of_year = df["OBSERVATION DATE"].dt.dayofyear

print(f"Date Range: [{df['OBSERVATION DATE'].min().strftime('%Y-%m-%d')} - {df['OBSERVATION DATE'].max().strftime('%Y-%m-%d')}]")
print(f"Latitude Range: [{df['LATITUDE'].min()} - {df['LATITUDE'].max()}]")
print(f"Longitude Range: [{df['LONGITUDE'].min()} - {df['LONGITUDE'].max()}]")

fig, ax = plt.subplots(figsize=(12, 8))
scatter = ax.scatter(
    df["LONGITUDE"], df["LATITUDE"],
    c=day_of_year, cmap="twilight_shifted", s=10, alpha=0.4, edgecolors="none",
)
cbar = plt.colorbar(scatter, ax=ax, pad=0.02)
cbar.set_label("Month")
cbar.set_ticks([1, 32, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335])
cbar.set_ticklabels(["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                      "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"])
ax.set_xlabel("Longitude")
ax.set_ylabel("Latitude")
ax.set_title("Sandhill Crane Sightings (by Month)")
ax.set_aspect("equal")
plt.tight_layout()
plt.savefig("figures/sightings_scatter.png", dpi=150)
plt.close()