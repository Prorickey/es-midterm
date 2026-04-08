use plotters::prelude::*;
use std::collections::HashMap;
use std::error::Error;

use crate::data::Observation;

const BAR_COLOR: RGBColor = RGBColor(70, 130, 180);

pub fn generate(obs: &[Observation], path: &str) -> Result<(), Box<dyn Error>> {
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for o in obs {
        if !o.state.is_empty() {
            *counts.entry(o.state.as_str()).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<(String, u32)> = counts
        .into_iter()
        .map(|(s, c)| (s.to_string(), c))
        .collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.truncate(25);
    // Reverse so the largest bar is at the top of the chart
    sorted.reverse();

    let max_count = sorted.last().map(|(_, c)| *c).unwrap_or(1);
    let n_bars = sorted.len() as u32;

    let (width, height) = (1800u32, 1200u32);
    let root =
        BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(30u32)
        .x_label_area_size(50u32)
        .y_label_area_size(200u32)
        .caption(
            "Top 25 States by Observation Count",
            ("sans-serif", 28),
        )
        .build_cartesian_2d(
            0u32..(max_count as f64 * 1.12) as u32,
            0u32..n_bars,
        )?;

    chart
        .configure_mesh()
        .x_desc("Number of Observations")
        .y_desc("")
        .disable_y_mesh()
        .y_label_formatter(&|v| {
            sorted
                .get(*v as usize)
                .map(|(name, _)| name.clone())
                .unwrap_or_default()
        })
        .y_labels(n_bars as usize)
        .x_label_formatter(&|v| {
            if *v >= 1_000_000 {
                format!("{:.1}M", *v as f64 / 1_000_000.0)
            } else if *v >= 1_000 {
                format!("{}K", v / 1_000)
            } else {
                format!("{}", v)
            }
        })
        .draw()?;

    for (i, (_state, count)) in sorted.iter().enumerate() {
        chart.draw_series(std::iter::once(Rectangle::new(
            [(0u32, i as u32), (*count, i as u32 + 1)],
            BAR_COLOR.filled(),
        )))?;
    }

    root.present()?;
    println!("Saved: {path}");

    Ok(())
}
