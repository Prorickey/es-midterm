use chrono::Datelike;
use plotters::prelude::*;
use std::error::Error;

use crate::colormap::blend_white;
use crate::data::{DataBounds, Observation};

const WINTER_COLOR: RGBColor = RGBColor(70, 130, 180);
const SUMMER_COLOR: RGBColor = RGBColor(205, 92, 92);

pub fn generate(obs: &[Observation], bounds: &DataBounds, path: &str, subtitle: &str) -> Result<(), Box<dyn Error>> {
    let winter: Vec<&Observation> = obs
        .iter()
        .filter(|o| matches!(o.date.month(), 12 | 1 | 2))
        .collect();
    let summer: Vec<&Observation> = obs
        .iter()
        .filter(|o| matches!(o.date.month(), 6 | 7 | 8))
        .collect();

    let (width, height) = (1800u32, 1200u32);
    let root = BitMapBackend::new(path, (width, height))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let panels = root.split_evenly((1, 2));

    let lat_pad = (bounds.max_lat - bounds.min_lat) * 0.02;
    let lon_pad = (bounds.max_lon - bounds.min_lon) * 0.02;

    let winter_title = format!("Winter (Dec\u{2013}Feb){subtitle}");
    let summer_title = format!("Summer (Jun\u{2013}Aug){subtitle}");
    let seasons = [
        (winter_title.as_str(), &winter, WINTER_COLOR),
        (summer_title.as_str(), &summer, SUMMER_COLOR),
    ];

    for (panel, (title, points, color)) in panels.iter().zip(seasons.iter()) {
        let blended = blend_white(*color, 0.6);

        let mut chart = ChartBuilder::on(panel)
            .margin(15u32)
            .x_label_area_size(40u32)
            .y_label_area_size(60u32)
            .caption(*title, ("sans-serif", 24))
            .build_cartesian_2d(
                (bounds.min_lon - lon_pad)..(bounds.max_lon + lon_pad),
                (bounds.min_lat - lat_pad)..(bounds.max_lat + lat_pad),
            )?;

        chart
            .configure_mesh()
            .x_desc("Longitude")
            .y_desc("Latitude")
            .draw()?;

        chart.draw_series(points.iter().map(|o| {
            Circle::new((o.lon, o.lat), 2, blended.filled())
        }))?;
    }

    root.present()?;
    println!("Saved: {path}");

    Ok(())
}
