use chrono::Datelike;
use plotters::prelude::*;
use std::error::Error;

use crate::data::Observation;

struct Category {
    label: &'static str,
    prefix: &'static str,
    color: RGBColor,
}

const CATEGORIES: &[Category] = &[
    Category {
        label: "Observed (C1)",
        prefix: "C1",
        color: RGBColor(100, 180, 220),
    },
    Category {
        label: "Possible (C2)",
        prefix: "C2",
        color: RGBColor(154, 205, 50),
    },
    Category {
        label: "Probable (C3)",
        prefix: "C3",
        color: RGBColor(230, 160, 50),
    },
    Category {
        label: "Confirmed (C4)",
        prefix: "C4",
        color: RGBColor(200, 50, 50),
    },
];

pub fn generate(obs: &[Observation], path: &str) -> Result<(), Box<dyn Error>> {
    let breeding_obs: Vec<&Observation> = obs
        .iter()
        .filter(|o| !o.breeding_code.is_empty())
        .collect();

    if breeding_obs.is_empty() {
        println!("No breeding records found, skipping breeding_latitude figure");
        return Ok(());
    }

    let lat_min = breeding_obs
        .iter()
        .map(|o| o.lat)
        .fold(f64::INFINITY, f64::min);
    let lat_max = breeding_obs
        .iter()
        .map(|o| o.lat)
        .fold(f64::NEG_INFINITY, f64::max);
    let lat_pad = (lat_max - lat_min) * 0.05;

    let (width, height) = (1800u32, 1200u32);
    let root =
        BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(30u32)
        .x_label_area_size(50u32)
        .y_label_area_size(70u32)
        .caption(
            "Breeding Evidence by Date and Latitude",
            ("sans-serif", 28),
        )
        .build_cartesian_2d(1u32..366u32, (lat_min - lat_pad)..(lat_max + lat_pad))?;

    chart
        .configure_mesh()
        .x_desc("Day of Year")
        .y_desc("Latitude (\u{00b0}N)")
        .x_label_formatter(&|v| {
            // Show month abbreviation at approximate day boundaries
            match *v {
                1 => "Jan".into(),
                32 => "Feb".into(),
                60 => "Mar".into(),
                91 => "Apr".into(),
                121 => "May".into(),
                152 => "Jun".into(),
                182 => "Jul".into(),
                213 => "Aug".into(),
                244 => "Sep".into(),
                274 => "Oct".into(),
                305 => "Nov".into(),
                335 => "Dec".into(),
                _ => format!("{}", v),
            }
        })
        .draw()?;

    for cat in CATEGORIES {
        let points: Vec<&Observation> = breeding_obs
            .iter()
            .filter(|o| o.breeding_category.starts_with(cat.prefix))
            .copied()
            .collect();

        if points.is_empty() {
            continue;
        }

        chart
            .draw_series(points.iter().map(|o| {
                let doy = o.date.ordinal();
                Circle::new((doy, o.lat), 4, cat.color.filled())
            }))?
            .label(format!("{} (n={})", cat.label, points.len()))
            .legend(move |(x, y)| Circle::new((x + 10, y), 5, cat.color.filled()));
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .margin(10)
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK.stroke_width(1))
        .label_font(("sans-serif", 16))
        .draw()?;

    root.present()?;
    println!(
        "Saved: {path} ({} breeding records)",
        breeding_obs.len()
    );

    Ok(())
}
