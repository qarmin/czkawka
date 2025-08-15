use polars_core::prelude::*;
use polars_io::prelude::*;
use std::fs;
use polars::prelude::GetOutput;

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

pub fn create_graphs() -> PolarsResult<()> {
    let df = load_dataframe_from_csv("compilation_results.txt")?;

    // Create output directory
    fs::create_dir_all("charts")?;

    // Plotting helper (stubbed out, as plotlars is not available)
    /*
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
        // ...plotting code...
        Ok(())
    }
    */

    // plot_barh calls removed or commented out
    /*
    plot_barh(
        &df,
        "Compilation Time (seconds)",
        "Compilation Time (seconds)",
        "Compilation Time by Config",
        "charts/compilation_time.png",
        "{:.1}s",
        1.0,
        false,
        Color::from("#1f77b4"),
    )?;
    // ...other plot_barh calls...
    */

    save_as_md(&df, "charts/compilation_results.md")?;

    Ok(())
}