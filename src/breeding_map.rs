use plotters::prelude::*;
use std::error::Error;

use crate::data::{DataBounds, Observation};

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

pub fn generate(obs: &[Observation], bounds: &DataBounds, path: &str) -> Result<(), Box<dyn Error>> {
    let breeding_obs: Vec<&Observation> = obs
        .iter()
        .filter(|o| !o.breeding_code.is_empty())
        .collect();

    let (width, height) = (1800u32, 1200u32);
    let root =
        BitMapBackend::new(path, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let lat_pad = (bounds.max_lat - bounds.min_lat) * 0.02;
    let lon_pad = (bounds.max_lon - bounds.min_lon) * 0.02;

    let mut chart = ChartBuilder::on(&root)
        .margin(30u32)
        .x_label_area_size(50u32)
        .y_label_area_size(70u32)
        .caption("Breeding Evidence Distribution", ("sans-serif", 28))
        .build_cartesian_2d(
            (bounds.min_lon - lon_pad)..(bounds.max_lon + lon_pad),
            (bounds.min_lat - lat_pad)..(bounds.max_lat + lat_pad),
        )?;

    chart
        .configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    // Draw each category in order so higher evidence draws on top
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
                Circle::new((o.lon, o.lat), 4, cat.color.filled())
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
