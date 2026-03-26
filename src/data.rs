use chrono::NaiveDate;
use std::error::Error;

pub struct Observation {
    pub lat: f64,
    pub lon: f64,
    pub date: NaiveDate,
    pub state: String,
    pub count: Option<u32>,
}

pub struct DataBounds {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
    pub min_date: NaiveDate,
    pub max_date: NaiveDate,
}

pub fn load_observations(path: &str) -> Result<(Vec<Observation>, DataBounds), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .flexible(true)
        .from_path(path)?;

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
    let state_idx = col("STATE")?;
    let count_idx = col("OBSERVATION COUNT")?;

    let mut observations: Vec<Observation> = Vec::new();
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
        let state = rec.get(state_idx).unwrap_or("").to_string();
        let count = rec.get(count_idx).and_then(|s| s.parse::<u32>().ok());

        min_date = Some(min_date.map_or(date, |d: NaiveDate| d.min(date)));
        max_date = Some(max_date.map_or(date, |d: NaiveDate| d.max(date)));
        min_lat = min_lat.min(lat);
        max_lat = max_lat.max(lat);
        min_lon = min_lon.min(lon);
        max_lon = max_lon.max(lon);

        observations.push(Observation {
            lat,
            lon,
            date,
            state,
            count,
        });
    }

    let bounds = DataBounds {
        min_lat,
        max_lat,
        min_lon,
        max_lon,
        min_date: min_date.ok_or("no observations found")?,
        max_date: max_date.ok_or("no observations found")?,
    };

    Ok((observations, bounds))
}
