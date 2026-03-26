# Sandhill Crane (*Antigone canadensis*) — Canadian eBird Observations

Analysis of Sandhill Crane observation data across Canada using the eBird Basic Dataset (February 2026 release).

## About

This project explores citizen-science observation records of the Sandhill Crane in Canada. The dataset includes information on observation counts, geographic locations, observation dates, survey effort, and observer metadata submitted through eBird.

## Data

The observation data is sourced from the [eBird Basic Dataset](https://ebird.org/science/use-ebird-data):

> eBird Basic Dataset. Version: EBD_relFeb-2026. Cornell Lab of Ornithology, Ithaca, New York. Feb 2026.

The dataset file (`data/ebd_sancra_relFeb-2026.tsv`) contains eBird records filtered for:

- **Species:** Sandhill Crane (*Antigone canadensis*)
- **Country:** Canada (CA)
- **Release:** February 2026

Supporting reference files are included in `data/`:

| File | Description |
|------|-------------|
| `BCRCodes.txt` | Bird Conservation Region codes |
| `IBACodes.txt` | Important Bird Area codes |
| `USFWSCodes.txt` | US Fish & Wildlife Service codes |
| `BirdLifeKBACodes.txt` | BirdLife Key Biodiversity Area codes |
| `Protocols.txt` | eBird observation protocol definitions |
| `eBird_Basic_Dataset_Metadata_v1.16.pdf` | Full dataset metadata documentation |
| `terms_of_use.txt` | eBird data access terms of use |
| `recommended_citation.txt` | Required citation for this dataset |

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition)

## Setup

```bash
cargo build
```

## Usage

```bash
cargo run
```

This parses the eBird TSV, prints date/latitude/longitude ranges, and saves a scatter plot colored by month to `figures/sightings_scatter.png`.

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
