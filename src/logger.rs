use log::{LevelFilter, Metadata, Record};

struct StdoutLogger {
    log_level: LevelFilter,
}

impl StdoutLogger {
    fn new(level: LevelFilter) -> Self {
        StdoutLogger { log_level: level }
    }
}

impl log::Log for StdoutLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        _metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args());
        }
    }
    fn flush(&self) {}
}

pub fn init(level: LevelFilter) {
    log::set_boxed_logger(Box::new(StdoutLogger::new(level)))
        .map(|()| log::set_max_level(level))
        .expect("Could not set logger");
}
