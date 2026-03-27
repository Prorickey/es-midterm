mod breeding_latitude;
mod breeding_map;
mod colormap;
mod data;
mod julian_date;
mod monthly_heatmap;
mod monthly_latitude;
mod nesting_latitude;
mod scatter;
mod seasonal_range;
mod state_bar;
mod yearly_monthly_latitude;

use std::error::Error;
use std::thread;

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

    let errors: Vec<String> = thread::scope(|s| {
        let handles: Vec<_> = vec![
            s.spawn(|| scatter::generate(&observations, &bounds).map_err(|e| e.to_string())),
            s.spawn(|| seasonal_range::generate(&observations, &bounds).map_err(|e| e.to_string())),
            s.spawn(|| monthly_heatmap::generate(&observations, &bounds).map_err(|e| e.to_string())),
            s.spawn(|| nesting_latitude::generate(&observations).map_err(|e| e.to_string())),
            s.spawn(|| monthly_latitude::generate(&observations).map_err(|e| e.to_string())),
            s.spawn(|| yearly_monthly_latitude::generate(&observations).map_err(|e| e.to_string())),
            s.spawn(|| julian_date::generate(&observations).map_err(|e| e.to_string())),
            s.spawn(|| state_bar::generate(&observations).map_err(|e| e.to_string())),
            s.spawn(|| breeding_map::generate(&observations, &bounds).map_err(|e| e.to_string())),
            s.spawn(|| breeding_latitude::generate(&observations).map_err(|e| e.to_string())),
        ];

        handles
            .into_iter()
            .filter_map(|h| match h.join() {
                Ok(Ok(())) => None,
                Ok(Err(e)) => Some(e),
                Err(_) => Some("thread panicked".to_string()),
            })
            .collect()
    });

    if !errors.is_empty() {
        for e in &errors {
            eprintln!("{e}");
        }
        return Err(format!("{} figure(s) failed", errors.len()).into());
    }

    Ok(())
}
