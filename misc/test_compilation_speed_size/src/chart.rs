use polars_core::prelude::*;
use polars_io::prelude::*;
use std::fs;
use plotters::prelude::*;

fn load_dataframe_from_csv(file_path: &str) -> PolarsResult<DataFrame> {
    let mut df = CsvReadOptions::default()
        .with_has_header(true)
        .map_parse_options(|opts| opts.with_separator(b'|'))
        .try_into_reader_with_file_path(Some(file_path.into()))?
        .finish()?;

    // Strip whitespace from string columns (bez unsafe)
    let col_names: Vec<String> = df.get_column_names().iter().map(|s| s.to_string()).collect();
    for name in col_names {
        if let Ok(col) = df.column(&name) {
            if matches!(col.dtype(), DataType::String) {
                let trimmed = col.str()?.apply(|v| v.map(|s| std::borrow::Cow::Owned(s.trim().to_string())));
                df.with_column(trimmed.into_series())?;
            }
        }
    }

    // Convert columns to numeric (only if column is String)
    let try_parse_numeric = |df: &mut DataFrame, col_name: &str| -> PolarsResult<()> {
        if let Ok(col) = df.column(col_name) {
            if matches!(col.dtype(), DataType::String) {
                let chunked = col
                    .str()?
                    .into_iter()
                    .map(|v| v.and_then(|s| s.parse::<f64>().ok()))
                    .collect::<Float64Chunked>();
                df.with_column(chunked.into_series())?;
            }
        }
        Ok(())
    };

    try_parse_numeric(&mut df, "Output File Size(in bytes)")?;
    try_parse_numeric(&mut df, "Target Folder Size(in bytes)")?;
    try_parse_numeric(&mut df, "Compilation Time(seconds)")?;
    try_parse_numeric(&mut df, "Rebuild Time(seconds)")?;

    Ok(df)
}

fn save_as_md(df: &DataFrame, path: &str) -> PolarsResult<()> {
    // Save markdown table (without types, shape, or empty columns)
    let columns: Vec<String> = df
        .get_column_names()
        .iter()
        .filter(|c| !c.contains('(') && !c.contains("Thread"))
        .map(|s| s.to_string())
        .collect();
    let mut subdf = df.select(&columns)?;

    // Remove last column if it's empty or all nulls
    let drop_col = subdf
        .get_columns()
        .last()
        .filter(|last| last.null_count() == last.len())
        .map(|last| last.name().to_string());
    if let Some(col_name) = drop_col {
        subdf.drop_in_place(&col_name)?;
    }

    // Write markdown manually (no types, no shape, no quotes for strings)
    let mut md = String::new();
    // Header
    let header: Vec<String> = subdf.get_column_names().iter().map(|s| s.to_string()).collect();
    md.push('|');
    md.push_str(&header.join("|"));
    md.push_str("|\n|");
    md.push_str(&header.iter().map(|_| "---").collect::<Vec<_>>().join("|"));
    md.push_str("|\n");
    // Rows
    for row in 0..subdf.height() {
        md.push('|');
        for (i, col) in subdf.get_columns().iter().enumerate() {
            let val = col.get(row);
            let s = match val {
                Ok(AnyValue::Null) => "".to_string(),
                Ok(v) => {
                    let mut s = v.to_string();
                    // Remove surrounding quotes if present
                    if s.starts_with('"') && s.ends_with('"') && s.len() > 1 {
                        s = s[1..s.len()-1].to_string();
                    }
                    s
                },
                Err(_) => "".to_string(),
            };
            md.push_str(&s);
            if i != subdf.width() - 1 {
                md.push('|');
            }
        }
        md.push_str("|\n");
    }
    std::fs::write(path, md)?;
    Ok(())
}

fn plot_barh(
    df: &DataFrame,
    value_col: &str,
    xlabel: &str,
    title: &str,
    filename: &str,
    _fmt: &str,
    unit_div: f64,
    dropna: bool,
    color: RGBColor,
) -> PolarsResult<()> {
    // Helper to find a Series by name, handling possible quotes and whitespace
    fn find_col<'a>(df: &'a DataFrame, name: &str) -> Option<&'a Series> {
        let name_trimmed = name.trim();
        println!("Columns {:?}", df.get_column_names());
        println!("Looking for column: {}", name_trimmed);
        df.get_columns().iter().find_map(|s| {
            let sname = s.name().trim();
            if sname == name_trimmed
                || sname == name_trimmed.replace('\"', "")
                || sname == format!("\"{}\"", name_trimmed)
            {
                Some(s)
            } else {
                None
            }
        }).and_then(|e|e.as_series())
    }

    let config_col = find_col(df, "BuildConfig")
        .or_else(|| find_col(df, "Config"))
        .ok_or_else(|| polars_core::error::PolarsError::ComputeError("Config column not found".into()))?;

    let value_col = find_col(df, value_col)
        .ok_or_else(|| polars_core::error::PolarsError::ComputeError(format!("{} not found", value_col).into()))?;

    // Use .str()? on Series
    let config_utf8 = config_col.str()?.into_iter();
    let value_f64 = value_col.f64()?.into_iter();

    let mut configs = Vec::new();
    let mut values = Vec::new();

    for (conf, val) in config_utf8.zip(value_f64) {
        if dropna && val.is_none() {
            continue;
        }
        configs.push(conf.unwrap_or("").to_string());
        values.push(val.unwrap_or(0.0) / unit_div);
    }

    // Sort descending by value
    let mut zipped: Vec<_> = configs.into_iter().zip(values.into_iter()).collect();
    zipped.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let (configs, values): (Vec<_>, Vec<_>) = zipped.into_iter().unzip();

    // Plot with plotters
    let root = BitMapBackend::new(filename, (1200, 900)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_val = values.iter().cloned().fold(0.0/0.0, f64::max);
    let x_max = if max_val.is_finite() && max_val > 0.0 { max_val * 1.15 } else { 1.0 };

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("Noto Sans", 30).into_font())
        .margin(40)
        .x_label_area_size(60)
        .y_label_area_size(300)
        .build_cartesian_2d(0f64..x_max, 0..configs.len())
        .unwrap();

    chart
        .configure_mesh()
        .x_desc(xlabel)
        .y_desc("")
        .y_labels(configs.len())
        .y_label_formatter(&|idx| configs.get(*idx).cloned().unwrap_or_default())
        .x_label_formatter(&|x| format!("{}", x))
        .axis_desc_style(("Noto Sans", 20))
        .label_style(("Noto Sans", 16))
        .draw()
        .unwrap();

    for (i, v) in values.iter().enumerate() {
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(0.0, i), (*v, i + 1)],
                color.filled(),
            )))
            .unwrap()
            .label(configs[i].clone())
            .legend(move |(x, y)| {
                Rectangle::new([(x, y - 5), (x + 20, y + 5)], color.filled())
            });

        // Draw value label
        chart
            .draw_series(std::iter::once(Text::new(
                format!("{}", v),
                (*v, i),
                ("Noto Sans", 16).into_font().color(&BLACK),
            )))
            .unwrap();
    }

    root.present().unwrap();

    Ok(())
}

pub fn create_graphs() -> PolarsResult<()> {
    let df = load_dataframe_from_csv("compilation_results.txt")?;

    // Create output directory
    fs::create_dir_all("charts")?;

    plot_barh(
        &df,
        "Compilation Time(seconds)",
        "Compilation Time (seconds)",
        "Compilation Time by Config",
        "charts/compilation_time.png",
        "{:.1}s",
        1.0,
        false,
        RGBColor(31, 119, 180),
    )?;
    plot_barh(
        &df,
        "Rebuild Time(seconds)",
        "Rebuild Time (seconds)",
        "Rebuild Time by Config",
        "charts/rebuild_time.png",
        "{:.1}s",
        1.0,
        false,
        RGBColor(255, 127, 14),
    )?;
    plot_barh(
        &df,
        "Output File Size(in bytes)",
        "Output File Size (MB)",
        "Output File Size by Config",
        "charts/output_file_size.png",
        "{:.1} MB",
        1024.0 * 1024.0,
        true,
        RGBColor(44, 160, 44),
    )?;
    plot_barh(
        &df,
        "Target Folder Size(bytes)",
        "Target Folder Size (GB)",
        "Target Folder Size by Config",
        "charts/target_folder_size.png",
        "{:.1} GB",
        1024.0 * 1024.0 * 1024.0,
        false,
        RGBColor(214, 39, 40),
    )?;

    save_as_md(&df, "charts/compilation_results.md")?;

    Ok(())
}