mod colormap;
mod data;
mod monthly_heatmap;
mod scatter;
mod seasonal_range;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (observations, bounds) =
        data::load_observations("data/ebd_CA_sancra_relFeb-2026.tsv")?;

    println!(
        "Date Range: [{} - {}]",
        bounds.min_date.format("%Y-%m-%d"),
        bounds.max_date.format("%Y-%m-%d"),
    );
    println!("Latitude Range: [{} - {}]", bounds.min_lat, bounds.max_lat);
    println!("Longitude Range: [{} - {}]", bounds.min_lon, bounds.max_lon);

    scatter::generate(&observations, &bounds)?;
    seasonal_range::generate(&observations, &bounds)?;
    monthly_heatmap::generate(&observations, &bounds)?;

    Ok(())
}
