use humantime;
use log::LevelFilter;
use std::time::SystemTime;

use polars::prelude::*;

use fern::colors::{Color, ColoredLevelConfig};
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Magenta)
        .trace(Color::Cyan);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{} {}] {}",
                colors.color(record.level()),
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

pub fn get_polars_df() {
    // Define data for each column
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 28];

// Create a Series for each column
    let names_series = Series::new("names", names);
    let ages_series = Series::new("ages", ages);

// Combine Series into a DataFrame
    let df = DataFrame::new(vec![names_series, ages_series]);

// Print the DataFrame
    println!("{:?}", df);
}
