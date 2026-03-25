use plotters::prelude::RGBColor;

/// Approximate matplotlib's `twilight_shifted` colormap (cyclic, 0.0–1.0 → RGB).
pub fn twilight_shifted(t: f64) -> RGBColor {
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

    let t = t - t.floor();
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
pub fn blend_white(c: RGBColor, alpha: f64) -> RGBColor {
    let mix = |ch: u8| (ch as f64 * alpha + 255.0 * (1.0 - alpha)).round() as u8;
    RGBColor(mix(c.0), mix(c.1), mix(c.2))
}

/// Sequential colormap from white to dark purple, for density heatmaps.
pub fn density_color(t: f64) -> RGBColor {
    let t = t.clamp(0.0, 1.0);
    #[rustfmt::skip]
    let stops: &[(f64, (u8, u8, u8))] = &[
        (0.0, (255, 255, 255)),
        (0.2, (218, 208, 235)),
        (0.4, (170, 140, 200)),
        (0.6, (128,  80, 165)),
        (0.8, ( 85,  30, 130)),
        (1.0, ( 40,  10,  70)),
    ];

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
    RGBColor(40, 10, 70)
}
