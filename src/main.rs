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

    let no_fl_owned: Vec<data::Observation> = observations
        .iter()
        .filter(|o| o.state != "Florida")
        .map(|o| data::Observation {
            lat: o.lat,
            lon: o.lon,
            date: o.date,
            state: o.state.clone(),
            count: o.count,
            breeding_code: o.breeding_code.clone(),
            breeding_category: o.breeding_category.clone(),
        })
        .collect();
    let no_fl_bounds = data::compute_bounds(&no_fl_owned)?;

    println!("=== All Observations ===");
    println!("  Observations: {}", observations.len());
    println!(
        "  Date Range: [{} - {}]",
        bounds.min_date.format("%Y-%m-%d"),
        bounds.max_date.format("%Y-%m-%d"),
    );
    println!("  Latitude Range: [{} - {}]", bounds.min_lat, bounds.max_lat);
    println!("  Longitude Range: [{} - {}]", bounds.min_lon, bounds.max_lon);

    println!("\n=== Excluding Florida ===");
    println!("  Observations: {}", no_fl_owned.len());
    println!(
        "  Date Range: [{} - {}]",
        no_fl_bounds.min_date.format("%Y-%m-%d"),
        no_fl_bounds.max_date.format("%Y-%m-%d"),
    );
    println!("  Latitude Range: [{} - {}]", no_fl_bounds.min_lat, no_fl_bounds.max_lat);
    println!("  Longitude Range: [{} - {}]", no_fl_bounds.min_lon, no_fl_bounds.max_lon);

    let errors: Vec<String> = thread::scope(|s| {
        let handles: Vec<_> = vec![
            // Full dataset
            s.spawn(|| scatter::generate(&observations, &bounds, "figures/sightings_scatter.png").map_err(|e| e.to_string())),
            s.spawn(|| seasonal_range::generate(&observations, &bounds, "figures/seasonal_range.png").map_err(|e| e.to_string())),
            s.spawn(|| monthly_heatmap::generate(&observations, &bounds, "figures/monthly_heatmap.png").map_err(|e| e.to_string())),
            s.spawn(|| nesting_latitude::generate(&observations, "figures/nesting_latitude.png").map_err(|e| e.to_string())),
            s.spawn(|| monthly_latitude::generate(&observations, "figures/monthly_latitude.png").map_err(|e| e.to_string())),
            s.spawn(|| yearly_monthly_latitude::generate(&observations, "figures/yearly_monthly_latitude.png").map_err(|e| e.to_string())),
            s.spawn(|| julian_date::generate(&observations, "figures/julian_date.png").map_err(|e| e.to_string())),
            s.spawn(|| state_bar::generate(&observations, "figures/state_observations.png").map_err(|e| e.to_string())),
            s.spawn(|| breeding_map::generate(&observations, &bounds, "figures/breeding_evidence.png").map_err(|e| e.to_string())),
            s.spawn(|| breeding_latitude::generate(&observations, "figures/breeding_latitude.png").map_err(|e| e.to_string())),
            // No-Florida dataset
            s.spawn(|| scatter::generate(&no_fl_owned, &no_fl_bounds, "figures/no_fl_sightings_scatter.png").map_err(|e| e.to_string())),
            s.spawn(|| seasonal_range::generate(&no_fl_owned, &no_fl_bounds, "figures/no_fl_seasonal_range.png").map_err(|e| e.to_string())),
            s.spawn(|| monthly_heatmap::generate(&no_fl_owned, &no_fl_bounds, "figures/no_fl_monthly_heatmap.png").map_err(|e| e.to_string())),
            s.spawn(|| nesting_latitude::generate(&no_fl_owned, "figures/no_fl_nesting_latitude.png").map_err(|e| e.to_string())),
            s.spawn(|| monthly_latitude::generate(&no_fl_owned, "figures/no_fl_monthly_latitude.png").map_err(|e| e.to_string())),
            s.spawn(|| yearly_monthly_latitude::generate(&no_fl_owned, "figures/no_fl_yearly_monthly_latitude.png").map_err(|e| e.to_string())),
            s.spawn(|| julian_date::generate(&no_fl_owned, "figures/no_fl_julian_date.png").map_err(|e| e.to_string())),
            s.spawn(|| state_bar::generate(&no_fl_owned, "figures/no_fl_state_observations.png").map_err(|e| e.to_string())),
            s.spawn(|| breeding_map::generate(&no_fl_owned, &no_fl_bounds, "figures/no_fl_breeding_evidence.png").map_err(|e| e.to_string())),
            s.spawn(|| breeding_latitude::generate(&no_fl_owned, "figures/no_fl_breeding_latitude.png").map_err(|e| e.to_string())),
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
