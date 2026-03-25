use chrono::{Datelike, NaiveDate};
use plotters::prelude::*;
use std::error::Error;

/// Approximate matplotlib's `twilight_shifted` colormap (cyclic, 0.0–1.0 → RGB).
fn twilight_shifted(t: f64) -> RGBColor {
    #[rustfmt::skip]
    let stops: &[(f64, (u8, u8, u8))] = &[
        (0.000, ( 48,  18,  60)),
        (0.125, (154,  13,  86)),
        (0.250, (231, 136, 146)),
        (0.375, (247, 226, 227)),
        (0.500, (221, 235, 237)),
        (0.625, (103, 177, 191)),
        (0.750, ( 51, 107, 137)),
        (0.875, ( 46,  55, 107)),
        (1.000, ( 48,  18,  60)),
    ];

    let t = t - t.floor(); // wrap to [0, 1)
    for w in stops.windows(2) {
        let (t0, c0) = w[0];
        let (t1, c1) = w[1];
        if t >= t0 && t <= t1 {
            let f = (t - t0) / (t1 - t0);
            return RGBColor(
                (c0.0 as f64 + f * (c1.0 as f64 - c0.0 as f64)).round() as u8,
                (c0.1 as f64 + f * (c1.1 as f64 - c0.1 as f64)).round() as u8,
                (c0.2 as f64 + f * (c1.2 as f64 - c0.2 as f64)).round() as u8,
            );
        }
    }
    RGBColor(stops[0].1 .0, stops[0].1 .1, stops[0].1 .2)
}

/// Blend `color` with white at the given alpha (matches matplotlib's `alpha=` on a white bg).
fn blend_white(c: RGBColor, alpha: f64) -> RGBColor {
    let mix = |ch: u8| (ch as f64 * alpha + 255.0 * (1.0 - alpha)).round() as u8;
    RGBColor(mix(c.0), mix(c.1), mix(c.2))
}

fn main() -> Result<(), Box<dyn Error>> {
    // ── Parse TSV ─────────────────────────────────────────────────────────────
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("data/ebd_CA_sancra_relFeb-2026.tsv")?;

    let headers = rdr.headers()?.clone();
    let col = |name: &str| -> Result<usize, Box<dyn Error>> {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| format!("missing column: {name}").into())
    };
    let date_idx = col("OBSERVATION DATE")?;
    let lat_idx = col("LATITUDE")?;
    let lon_idx = col("LONGITUDE")?;

    let mut points: Vec<(f64, f64, u32)> = Vec::new(); // (lon, lat, day_of_year)
    let mut min_date: Option<NaiveDate> = None;
    let mut max_date: Option<NaiveDate> = None;
    let mut min_lat = f64::INFINITY;
    let mut max_lat = f64::NEG_INFINITY;
    let mut min_lon = f64::INFINITY;
    let mut max_lon = f64::NEG_INFINITY;

    for result in rdr.records() {
        let rec = result?;

        let date = match rec.get(date_idx) {
            Some(s) if !s.is_empty() => match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => continue,
            },
            _ => continue,
        };
        let lat: f64 = match rec.get(lat_idx).and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let lon: f64 = match rec.get(lon_idx).and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => continue,
        };

        min_date = Some(min_date.map_or(date, |d| d.min(date)));
        max_date = Some(max_date.map_or(date, |d| d.max(date)));
        min_lat = min_lat.min(lat);
        max_lat = max_lat.max(lat);
        min_lon = min_lon.min(lon);
        max_lon = max_lon.max(lon);
        points.push((lon, lat, date.ordinal()));
    }

    println!(
        "Date Range: [{} - {}]",
        min_date.unwrap().format("%Y-%m-%d"),
        max_date.unwrap().format("%Y-%m-%d"),
    );
    println!("Latitude Range: [{} - {}]", min_lat, max_lat);
    println!("Longitude Range: [{} - {}]", min_lon, max_lon);

    // ── Plot ──────────────────────────────────────────────────────────────────
    // 12 × 8 in at 150 dpi → 1800 × 1200 px
    let (width, height) = (1800u32, 1200u32);
    let cbar_strip = 130u32;

    let root = BitMapBackend::new("figures/sightings_scatter.png", (width, height))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let (plot_area, cbar_area) = root.split_horizontally((width - cbar_strip) as i32);

    let lat_pad = (max_lat - min_lat) * 0.02;
    let lon_pad = (max_lon - min_lon) * 0.02;

    let mut chart = ChartBuilder::on(&plot_area)
        .margin(20u32)
        .x_label_area_size(50u32)
        .y_label_area_size(70u32)
        .caption("Sandhill Crane Sightings (by Month)", ("sans-serif", 28))
        .build_cartesian_2d(
            (min_lon - lon_pad)..(max_lon + lon_pad),
            (min_lat - lat_pad)..(max_lat + lat_pad),
        )?;

    chart
        .configure_mesh()
        .x_desc("Longitude")
        .y_desc("Latitude")
        .draw()?;

    chart.draw_series(points.iter().map(|&(lon, lat, doy)| {
        let t = (doy as f64 - 1.0) / 365.0;
        let color = blend_white(twilight_shifted(t), 0.4);
        Circle::new((lon, lat), 2, color.filled())
    }))?;

    // ── Colorbar ──────────────────────────────────────────────────────────────
    // Vertical bar: Jan at bottom (t=0), Dec at top (t=1), matching matplotlib default.
    let cb_x = 15i32;
    let cb_w = 20i32;
    let cb_top = 80i32;
    let cb_bot = height as i32 - 60;
    let cb_h = cb_bot - cb_top;

    // Gradient strip
    for i in 0..cb_h {
        // i=0 is the top of the colorbar → t=1 (Dec); i=cb_h-1 → t=0 (Jan)
        let t = (cb_h - 1 - i) as f64 / (cb_h - 1) as f64;
        let color = twilight_shifted(t);
        cbar_area.draw(&Rectangle::new(
            [(cb_x, cb_top + i), (cb_x + cb_w, cb_top + i + 1)],
            color.filled(),
        ))?;
    }

    // Border
    cbar_area.draw(&Rectangle::new(
        [(cb_x, cb_top), (cb_x + cb_w, cb_bot)],
        BLACK.stroke_width(1),
    ))?;

    // Month ticks (day-of-year → y position, Jan at bottom)
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

    // "Month" label above the colorbar
    cbar_area.draw(&Text::new("Month", (cb_x, cb_top - 22), ("sans-serif", 15)))?;

    root.present()?;
    println!("Saved: figures/sightings_scatter.png");

    Ok(())
}
