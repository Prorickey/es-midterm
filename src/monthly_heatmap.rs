use chrono::Datelike;
use plotters::prelude::*;
use std::error::Error;

use crate::colormap::density_color;
use crate::data::{DataBounds, Observation};

const BIN_SIZE: f64 = 0.5;

const MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

fn bin_observations(
    obs: &[Observation],
    bounds: &DataBounds,
) -> (Vec<Vec<u32>>, usize, usize) {
    let n_lon = ((bounds.max_lon - bounds.min_lon) / BIN_SIZE).ceil() as usize + 1;
    let n_lat = ((bounds.max_lat - bounds.min_lat) / BIN_SIZE).ceil() as usize + 1;
    let mut grid = vec![vec![0u32; n_lon]; n_lat];

    for o in obs {
        let lat_bin = ((o.lat - bounds.min_lat) / BIN_SIZE) as usize;
        let lon_bin = ((o.lon - bounds.min_lon) / BIN_SIZE) as usize;
        let lat_bin = lat_bin.min(n_lat - 1);
        let lon_bin = lon_bin.min(n_lon - 1);
        grid[lat_bin][lon_bin] += 1;
    }

    (grid, n_lat, n_lon)
}

pub fn generate(obs: &[Observation], bounds: &DataBounds) -> Result<(), Box<dyn Error>> {
    let (width, height) = (2400u32, 1800u32);
    let root = BitMapBackend::new("figures/monthly_heatmap.png", (width, height))
        .into_drawing_area();
    root.fill(&WHITE)?;

    root.titled("Sandhill Crane Sightings by Month", ("sans-serif", 32))?;

    // Group observations by month
    let mut by_month: [Vec<&Observation>; 12] = Default::default();
    for o in obs {
        let idx = (o.date.month() - 1) as usize;
        by_month[idx].push(o);
    }

    // Find global max for consistent color scale
    let global_max = by_month
        .iter()
        .map(|month_obs| {
            let (grid, _, _) = bin_observations(
                &month_obs.iter().map(|&&ref o| Observation {
                    lat: o.lat,
                    lon: o.lon,
                    date: o.date,
                    state: o.state.clone(),
                    count: o.count,
                    breeding_code: o.breeding_code.clone(),
                    breeding_category: o.breeding_category.clone(),
                }).collect::<Vec<_>>(),
                bounds,
            );
            grid.iter()
                .flat_map(|row| row.iter())
                .copied()
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(1);
    let log_max = (global_max as f64 + 1.0).ln();

    let lat_pad = (bounds.max_lat - bounds.min_lat) * 0.02;
    let lon_pad = (bounds.max_lon - bounds.min_lon) * 0.02;

    let panels = root.split_evenly((3, 4));

    for (idx, panel) in panels.iter().enumerate() {
        let month_obs: Vec<Observation> = by_month[idx]
            .iter()
            .map(|&o| Observation {
                lat: o.lat,
                lon: o.lon,
                date: o.date,
                state: o.state.clone(),
                count: o.count,
                breeding_code: o.breeding_code.clone(),
                breeding_category: o.breeding_category.clone(),
            })
            .collect();

        let (grid, n_lat, n_lon) = bin_observations(&month_obs, bounds);

        let mut chart = ChartBuilder::on(panel)
            .margin(8u32)
            .x_label_area_size(30u32)
            .y_label_area_size(50u32)
            .caption(MONTH_NAMES[idx], ("sans-serif", 20))
            .build_cartesian_2d(
                (bounds.min_lon - lon_pad)..(bounds.max_lon + lon_pad),
                (bounds.min_lat - lat_pad)..(bounds.max_lat + lat_pad),
            )?;

        chart.configure_mesh().disable_mesh().draw()?;

        for lat_bin in 0..n_lat {
            for lon_bin in 0..n_lon {
                let count = grid[lat_bin][lon_bin];
                if count == 0 {
                    continue;
                }
                let t = (count as f64 + 1.0).ln() / log_max;
                let color = density_color(t);

                let x0 = bounds.min_lon + lon_bin as f64 * BIN_SIZE;
                let y0 = bounds.min_lat + lat_bin as f64 * BIN_SIZE;

                chart.draw_series(std::iter::once(Rectangle::new(
                    [(x0, y0), (x0 + BIN_SIZE, y0 + BIN_SIZE)],
                    color.filled(),
                )))?;
            }
        }
    }

    root.present()?;
    println!("Saved: figures/monthly_heatmap.png");

    Ok(())
}
