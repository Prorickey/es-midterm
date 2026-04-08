use chrono::Datelike;
use plotters::prelude::*;
use std::error::Error;

use crate::data::Observation;

const MONTH_LABELS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn generate(obs: &[Observation], path: &str) -> Result<(), Box<dyn Error>> {
    // Compute mean latitude per month (1..=12)
    let mut month_sums = [0.0f64; 12];
    let mut month_counts = [0u64; 12];

    for o in obs {
        let idx = (o.date.month() - 1) as usize;
        month_sums[idx] += o.lat;
        month_counts[idx] += 1;
    }

    let points: Vec<(f64, f64)> = (0..12)
        .filter(|&i| month_counts[i] > 0)
        .map(|i| {
            let month = (i + 1) as f64;
            let mean_lat = month_sums[i] / month_counts[i] as f64;
            (month, mean_lat)
        })
        .collect();

    if points.len() < 2 {
        return Err("not enough monthly data to plot".into());
    }

    // Axis ranges
    let lat_min = points
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::INFINITY, f64::min);
    let lat_max = points
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::NEG_INFINITY, f64::max);
    let lat_pad = (lat_max - lat_min) * 0.15;

    let (width, height) = (1800u32, 1200u32);
    let root =
        BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(30u32)
        .x_label_area_size(50u32)
        .y_label_area_size(70u32)
        .caption("Mean Latitude by Month", ("sans-serif", 28))
        .build_cartesian_2d(0.5f64..12.5f64, (lat_min - lat_pad)..(lat_max + lat_pad))?;

    chart
        .configure_mesh()
        .x_desc("Month")
        .y_desc("Mean Latitude (\u{00b0}N)")
        .x_label_formatter(&|v| {
            let i = v.round() as usize;
            if (1..=12).contains(&i) {
                MONTH_LABELS[i - 1].to_string()
            } else {
                String::new()
            }
        })
        .x_labels(12)
        .draw()?;

    // Connecting curve (cubic spline approximation via Catmull-Rom)
    let curve_points = catmull_rom_chain(&points, 20);
    chart.draw_series(LineSeries::new(
        curve_points,
        RGBColor(70, 130, 180).stroke_width(2),
    ))?;

    // Scatter points
    chart.draw_series(points.iter().map(|&(x, y)| {
        Circle::new((x, y), 6, RGBColor(70, 130, 180).filled())
    }))?;

    root.present()?;
    println!("Saved: {path}");

    Ok(())
}

/// Generate a smooth curve through the given points using Catmull-Rom interpolation.
/// `steps` is the number of interpolated segments between each pair of control points.
fn catmull_rom_chain(points: &[(f64, f64)], steps: usize) -> Vec<(f64, f64)> {
    let n = points.len();
    if n < 2 {
        return points.to_vec();
    }

    let mut result = Vec::with_capacity((n - 1) * steps + 1);

    for i in 0..n - 1 {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < n { points[i + 2] } else { points[n - 1] };

        for s in 0..steps {
            let t = s as f64 / steps as f64;
            let x = catmull_rom(t, p0.0, p1.0, p2.0, p3.0);
            let y = catmull_rom(t, p0.1, p1.1, p2.1, p3.1);
            result.push((x, y));
        }
    }
    result.push(*points.last().unwrap());

    result
}

fn catmull_rom(t: f64, p0: f64, p1: f64, p2: f64, p3: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}
