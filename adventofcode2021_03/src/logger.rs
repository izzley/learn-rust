use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().to_rfc3339();
            let module = record.module_path().unwrap_or_default();
            let function = record.file().unwrap_or_default();
            println!(
                "{} {} [{}] [{}] - {}",
                timestamp,
                Local::now().format("%Z"),
                module,
                function,
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    // initialize logger to output to stdout at info
    log::set_logger(&SimpleLogger).map(|()| log::set_max_level(LevelFilter::Info))
}
