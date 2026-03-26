use chrono::Datelike;
use plotters::prelude::*;
use std::collections::BTreeMap;
use std::error::Error;

use crate::data::Observation;

pub fn generate(obs: &[Observation]) -> Result<(), Box<dyn Error>> {
    // Group November observations by year, compute mean latitude per year
    let mut year_lats: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
    for o in obs.iter().filter(|o| o.date.month() == 11) {
        year_lats.entry(o.date.year()).or_default().push(o.lat);
    }

    let points: Vec<(f64, f64)> = year_lats
        .iter()
        .map(|(&year, lats)| {
            let mean = lats.iter().sum::<f64>() / lats.len() as f64;
            (year as f64, mean)
        })
        .collect();

    if points.len() < 2 {
        return Err("not enough November data to plot".into());
    }

    // Linear regression: y = slope * x + intercept
    let n = points.len() as f64;
    let sum_x: f64 = points.iter().map(|(x, _)| x).sum();
    let sum_y: f64 = points.iter().map(|(_, y)| y).sum();
    let sum_xx: f64 = points.iter().map(|(x, _)| x * x).sum();
    let sum_xy: f64 = points.iter().map(|(x, y)| x * y).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    // R-squared
    let y_mean = sum_y / n;
    let ss_tot: f64 = points.iter().map(|(_, y)| (y - y_mean).powi(2)).sum();
    let ss_res: f64 = points
        .iter()
        .map(|(x, y)| (y - (slope * x + intercept)).powi(2))
        .sum();
    let r_squared = 1.0 - ss_res / ss_tot;

    // Axis ranges
    let x_min = points.first().unwrap().0 - 1.0;
    let x_max = points.last().unwrap().0 + 1.0;
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
        BitMapBackend::new("figures/nesting_latitude.png", (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(30u32)
        .x_label_area_size(50u32)
        .y_label_area_size(70u32)
        .caption(
            "Mean November Latitude by Year",
            ("sans-serif", 28),
        )
        .build_cartesian_2d(x_min..x_max, (lat_min - lat_pad)..(lat_max + lat_pad))?;

    chart
        .configure_mesh()
        .x_desc("Year")
        .y_desc("Mean Latitude (\u{00b0}N)")
        .x_label_formatter(&|v| format!("{}", *v as i32))
        .draw()?;

    // Scatter points
    chart.draw_series(points.iter().map(|&(x, y)| {
        Circle::new((x, y), 5, RGBColor(70, 130, 180).filled())
    }))?;

    // Line of best fit
    let line_y0 = slope * x_min + intercept;
    let line_y1 = slope * x_max + intercept;
    chart.draw_series(LineSeries::new(
        vec![(x_min, line_y0), (x_max, line_y1)],
        RED.stroke_width(2),
    ))?;

    // Annotation: equation and R^2
    let eq_label = format!(
        "y = {:.4}x {} {:.2}",
        slope,
        if intercept >= 0.0 { "+" } else { "\u{2212}" },
        intercept.abs(),
    );
    let r2_label = format!("R\u{00b2} = {:.4}", r_squared);

    let text_x = x_min + (x_max - x_min) * 0.05;
    let text_y_top = lat_max + lat_pad * 0.6;
    let text_y_bot = text_y_top - lat_pad * 0.35;

    chart.draw_series(std::iter::once(Text::new(
        eq_label,
        (text_x, text_y_top),
        ("sans-serif", 20).into_font().color(&RED),
    )))?;
    chart.draw_series(std::iter::once(Text::new(
        r2_label,
        (text_x, text_y_bot),
        ("sans-serif", 20).into_font().color(&RED),
    )))?;

    root.present()?;
    println!("Saved: figures/nesting_latitude.png");

    Ok(())
}
