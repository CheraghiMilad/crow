// logger.rs
use env_logger;
use log::LevelFilter;

pub fn init_logger() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
}