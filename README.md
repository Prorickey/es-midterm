# Sandhill Crane (*Antigone canadensis*) — eBird Observations

Analysis of Sandhill Crane observation data using the eBird Basic Dataset (February 2026 release). The dataset spans 2.76 million observations from 1925 to 2026.

## Figures

Each figure is generated for the full dataset and a second version excluding Florida observations. All outputs are saved to `figures/`.

| Figure | All Data | Excluding Non-migratory Sandhill Cranes (Excludes Florida) |
|--------|----------|-------------------|
| Sightings Scatter (by Month) | [sightings_scatter.png](figures/sightings_scatter.png) | [no_fl_sightings_scatter.png](figures/no_fl_sightings_scatter.png) |
| Seasonal Range (Winter vs Summer) | [seasonal_range.png](figures/seasonal_range.png) | [no_fl_seasonal_range.png](figures/no_fl_seasonal_range.png) |
| Monthly Sightings Heatmap | [monthly_heatmap.png](figures/monthly_heatmap.png) | [no_fl_monthly_heatmap.png](figures/no_fl_monthly_heatmap.png) |
| Mean Latitude by Month | [monthly_latitude.png](figures/monthly_latitude.png) | [no_fl_monthly_latitude.png](figures/no_fl_monthly_latitude.png) |
| Monthly Mean Latitude Over Time | [yearly_monthly_latitude.png](figures/yearly_monthly_latitude.png) | [no_fl_yearly_monthly_latitude.png](figures/no_fl_yearly_monthly_latitude.png) |
| Mean November Latitude by Year | [nesting_latitude.png](figures/nesting_latitude.png) | [no_fl_nesting_latitude.png](figures/no_fl_nesting_latitude.png) |
| Julian Day of Observation by Year | [julian_date.png](figures/julian_date.png) | [no_fl_julian_date.png](figures/no_fl_julian_date.png) |
| Top 25 States by Observation Count | [state_observations.png](figures/state_observations.png) | [no_fl_state_observations.png](figures/no_fl_state_observations.png) |
| Breeding Evidence Distribution | [breeding_evidence.png](figures/breeding_evidence.png) | [no_fl_breeding_evidence.png](figures/no_fl_breeding_evidence.png) |
| Breeding Evidence by Date & Latitude | [breeding_latitude.png](figures/breeding_latitude.png) | [no_fl_breeding_latitude.png](figures/no_fl_breeding_latitude.png) |

## Data

The observation data is sourced from the [eBird Basic Dataset](https://ebird.org/science/use-ebird-data):

> eBird Basic Dataset. Version: EBD_relFeb-2026. Cornell Lab of Ornithology, Ithaca, New York. Feb 2026.

The dataset contains eBird records filtered for:

- **Species:** Sandhill Crane (*Antigone canadensis*)
- **Release:** February 2026

Records before 1925 and observations with positive longitudes (Eastern Hemisphere) are excluded during loading.

## Building & Running

**Requirements:** [Rust](https://www.rust-lang.org/tools/install) (2021 edition)

```bash
# Build and generate all figures
./generate.sh

# Or manually
cargo build --release
./target/release/es-midterm
```

## Data Terms of Use

The eBird data included in this project is provided by the Cornell Lab of Ornithology under their [eBird Data Access Terms of Use](https://www.birds.cornell.edu/home/ebird-data-access-terms-of-use/). Key conditions:

- Data is supplied **only for applied and basic research and education**
- **Commercial use is prohibited** without prior written permission
- Users must provide **full and appropriate acknowledgement and citation**
- Original datasets must **not be redistributed**; link to the source instead
- Derived products must carry the **same terms of use**

For full terms, see `data/terms_of_use.txt`.

## Citation

If you use or reference this analysis, please cite the underlying data:

> eBird Basic Dataset. Version: EBD_relFeb-2026. Cornell Lab of Ornithology, Ithaca, New York. Feb 2026.

## License

The **analysis code** in this repository is licensed under the [MIT License](LICENSE).

The **eBird data** (`data/` directory) is provided under the Cornell Lab of Ornithology's terms of use and is **not** covered by the MIT License. See `data/terms_of_use.txt` for details.
