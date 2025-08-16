use plotters::prelude::*;
use std::fs::{self, File};
use std::io::{BufRead, BufReader}; use plotters::style::text_anchor::Pos;
use plotters::style::text_anchor::VPos; use plotters::style::text_anchor::HPos;

pub fn create_chart() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare output directory
    fs::create_dir_all("charts")?;

    // Open and read the file
    let file = File::open("compilation_results.txt")?;
    let reader = BufReader::new(file);

    // Read header and find column indices
    let mut lines = reader.lines();
    let header = lines.next().unwrap()?;
    let headers: Vec<&str> = header.split('|').collect();
    let config_idx = headers.iter().position(|&h| h.trim() == "BuildConfig").unwrap();
    let time_idx = headers.iter().position(|&h| h.trim() == "Compilation Time(seconds)").unwrap();

    // Parse data
    let mut data = Vec::new();
    for line in lines {
        let line = line?;
        let cols: Vec<&str> = line.split('|').collect();
        if cols.len() <= time_idx { continue; }
        let config = cols[config_idx].trim();
        let time: f64 = cols[time_idx].trim().parse().unwrap_or(0.0);
        data.push((config.to_string(), time));
    }
    dbg!(&data);

    // Sort by time descending
    data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Plot
    let root = BitMapBackend::new("charts/compilation_time.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_time = data.iter().map(|(_, t)| *t).fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Compilation Time by Config", ("Noto Sans", 50))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(300)
        .build_cartesian_2d(0f64..(max_time * 1.1), 0..data.len())?;

    chart
        .configure_mesh()
        .x_desc("Compilation Time (seconds)")
        .y_labels(data.len())
        .y_label_formatter(&|idx| {
            if let Some((label, _)) = data.get(*idx) {
                label.clone()
            } else {
                "".to_string()
            }
        })
        .y_label_style(("Noto Sans", 30).into_font().style(FontStyle::Bold))
        .x_label_style(("Noto Sans", 30).into_font())
        .draw()?;

    chart.draw_series(
        data.iter().enumerate().map(|(i, (_label, value))| {
            Rectangle::new(
                [(0.0, i), (*value, i + 1)],
                BLUE.filled(),
            )
        }),
    )?;

    root.present()?;
    Ok(())
}