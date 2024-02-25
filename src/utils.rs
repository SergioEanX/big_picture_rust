use humantime;
use log::LevelFilter;
use std::time::SystemTime;

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
