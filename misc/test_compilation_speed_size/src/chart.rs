use polars::prelude::*;
use plotlars::{BarChart, Chart, Color, Font, LabelPosition};
use std::fs;
use std::path::Path;

struct Color(&'static str);

pub fn create_graphs() -> PolarsResult<()> {
    // Read the data
    let mut df = CsvReader::from_path("compilation_results.txt")?
        .has_header(true)
        .with_delimiter(b'|')
        .finish()?;

    // Strip whitespace from string columns
    for col in df.get_columns_mut() {
        if let DataType::Utf8 = col.dtype() {
            let s = col.utf8()?.apply(|v| v.trim(), GetOutput::from_type(DataType::Utf8));
            *col = s.into_series();
        }
    }

    // Convert columns to numeric
    let output_file_size = df
        .column("Output File Size(in bytes)")?
        .utf8()?
        .as_binary()
        .iter()
        .map(|v| v.and_then(|s| std::str::from_utf8(s).ok()?.parse::<f64>().ok()))
        .collect::<Float64Chunked>();
    df.with_column(output_file_size.into_series().rename("Output File Size (bytes)"))?;

    let target_folder_size = df
        .column("Target Folder Size(in bytes)")?
        .utf8()?
        .as_binary()
        .iter()
        .map(|v| v.and_then(|s| std::str::from_utf8(s).ok()?.parse::<f64>().ok()))
        .collect::<Float64Chunked>();
    df.with_column(target_folder_size.into_series().rename("Target Folder Size (bytes)"))?;

    let compilation_time = df
        .column("Compilation Time(seconds)")?
        .utf8()?
        .as_binary()
        .iter()
        .map(|v| v.and_then(|s| std::str::from_utf8(s).ok()?.parse::<f64>().ok()))
        .collect::<Float64Chunked>();
    df.with_column(compilation_time.into_series().rename("Compilation Time (seconds)"))?;

    let rebuild_time = df
        .column("Rebuild Time(seconds)")?
        .utf8()?
        .as_binary()
        .iter()
        .map(|v| v.and_then(|s| std::str::from_utf8(s).ok()?.parse::<f64>().ok()))
        .collect::<Float64Chunked>();
    df.with_column(rebuild_time.into_series().rename("Rebuild Time (seconds)"))?;

    // Create output directory
    fs::create_dir_all("charts")?;

    // Plotting helper
    fn plot_barh(
        df: &DataFrame,
        value_col: &str,
        xlabel: &str,
        title: &str,
        filename: &str,
        fmt: &str,
        unit_div: f64,
        dropna: bool,
        color: Color,
    ) -> PolarsResult<()> {
        let config = df.column("BuildConfig")?.utf8()?;
        let values = df.column(value_col)?.f64()?;

        let mut data: Vec<(&str, f64)> = config
            .into_iter()
            .zip(values)
            .filter_map(|(c, v)| {
                let v = v.map(|v| v / unit_div);
                if dropna && v.is_none() {
                    None
                } else {
                    Some((c.unwrap_or(""), v.unwrap_or(0.0)))
                }
            })
            .collect();

        data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let labels: Vec<String> = data.iter().map(|(c, _)| c.to_string()).collect();
        let values: Vec<f64> = data.iter().map(|(_, v)| *v).collect();

        let mut chart = BarChart::new(&labels, &values)
            .horizontal()
            .bar_color(color)
            .label_position(LabelPosition::End)
            .label_format(fmt)
            .title(title)
            .xlabel(xlabel)
            .font(Font::new("Noto Sans", 13.0));

        chart.save(Path::new(filename)).unwrap();
        Ok(())
    }

    plot_barh(
        &df,
        "Compilation Time (seconds)",
        "Compilation Time (seconds)",
        "Compilation Time by Config",
        "charts/compilation_time.png",
        "{:.1}s",
        1.0,
        false,
        Color::from_hex("#1f77b4"),
    )?;
    plot_barh(
        &df,
        "Rebuild Time (seconds)",
        "Rebuild Time (seconds)",
        "Rebuild Time by Config",
        "charts/rebuild_time.png",
        "{:.1}s",
        1.0,
        false,
        Color::from_hex("#1f77b4"),
    )?;
    plot_barh(
        &df,
        "Output File Size (bytes)",
        "Output File Size (MB)",
        "Output File Size by Config",
        "charts/output_file_size.png",
        "{:.1} MB",
        1024.0 * 1024.0,
        true,
        Color::from_hex("#1f77b4"),
    )?;
    plot_barh(
        &df,
        "Target Folder Size (bytes)",
        "Target Folder Size (GB)",
        "Target Folder Size by Config",
        "charts/target_folder_size.png",
        "{:.1} GB",
        1024.0 * 1024.0 * 1024.0,
        false,
        Color::from_hex("#1f77b4"),
    )?;

    // Save markdown table
    let columns: Vec<&str> = df
        .get_column_names()
        .iter()
        .filter(|c| !c.contains('(') && !c.contains("Thread"))
        .map(|s| s.as_str())
        .collect();
    let subdf = df.select(&columns)?;
    let md = subdf.to_string();
    std::fs::write("charts/compilation_results.md", md)?;

    Ok(())
}