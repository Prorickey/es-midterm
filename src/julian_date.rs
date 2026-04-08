use chrono::Datelike;
use plotters::prelude::*;
use std::error::Error;

use crate::colormap::{blend_white, twilight_shifted};
use crate::data::Observation;

pub fn generate(obs: &[Observation], path: &str) -> Result<(), Box<dyn Error>> {
    let points: Vec<(f64, f64)> = obs
        .iter()
        .map(|o| (o.date.year() as f64, o.date.ordinal() as f64))
        .collect();

    if points.is_empty() {
        return Err("no observations to plot".into());
    }

    let x_min = points.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min) - 1.0;
    let x_max = points.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max) + 1.0;

    let (width, height) = (1800u32, 1200u32);
    let cbar_strip = 130u32;
    let root =
        BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let (plot_area, cbar_area) = root.split_horizontally((width - cbar_strip) as i32);

    let mut chart = ChartBuilder::on(&plot_area)
        .margin(20u32)
        .x_label_area_size(50u32)
        .y_label_area_size(70u32)
        .caption("Julian Day of Observation by Year", ("sans-serif", 28))
        .build_cartesian_2d(x_min..x_max, 0.0f64..366.0f64)?;

    chart
        .configure_mesh()
        .x_desc("Year")
        .y_desc("Julian Day")
        .x_label_formatter(&|v| format!("{}", *v as i32))
        .draw()?;

    chart.draw_series(points.iter().map(|&(x, y)| {
        let t = (y - 1.0) / 365.0;
        let color = blend_white(twilight_shifted(t), 0.8);
        Circle::new((x, y), 2, color.filled())
    }))?;

    // Colorbar
    let cb_x = 15i32;
    let cb_w = 20i32;
    let cb_top = 80i32;
    let cb_bot = height as i32 - 60;
    let cb_h = cb_bot - cb_top;

    for i in 0..cb_h {
        let t = (cb_h - 1 - i) as f64 / (cb_h - 1) as f64;
        let color = twilight_shifted(t);
        cbar_area.draw(&Rectangle::new(
            [(cb_x, cb_top + i), (cb_x + cb_w, cb_top + i + 1)],
            color.filled(),
        ))?;
    }
    cbar_area.draw(&Rectangle::new(
        [(cb_x, cb_top), (cb_x + cb_w, cb_bot)],
        BLACK.stroke_width(1),
    ))?;

    let months = [
        ("Jan", 1u32),
        ("Feb", 32),
        ("Mar", 60),
        ("Apr", 91),
        ("May", 121),
        ("Jun", 152),
        ("Jul", 182),
        ("Aug", 213),
        ("Sep", 244),
        ("Oct", 274),
        ("Nov", 305),
        ("Dec", 335),
    ];
    for (label, doy) in &months {
        let t = (*doy as f64 - 1.0) / 365.0;
        let y = cb_bot - (t * cb_h as f64).round() as i32;

        cbar_area.draw(&PathElement::new(
            vec![(cb_x + cb_w, y), (cb_x + cb_w + 5, y)],
            BLACK.stroke_width(1),
        ))?;
        cbar_area.draw(&Text::new(
            *label,
            (cb_x + cb_w + 8, y - 7),
            ("sans-serif", 14),
        ))?;
    }

    cbar_area.draw(&Text::new("Month", (cb_x, cb_top - 22), ("sans-serif", 15)))?;

    root.present()?;
    println!("Saved: {path}");

    Ok(())
}
