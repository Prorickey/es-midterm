mod colormap;
mod data;
mod monthly_heatmap;
mod monthly_latitude;
mod nesting_latitude;
mod scatter;
mod seasonal_range;
mod yearly_monthly_latitude;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (observations, bounds) =
        data::load_observations("data/ebd_sancra_relFeb-2026.tsv")?;

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
    nesting_latitude::generate(&observations)?;
    monthly_latitude::generate(&observations)?;
    yearly_monthly_latitude::generate(&observations)?;

    Ok(())
}
