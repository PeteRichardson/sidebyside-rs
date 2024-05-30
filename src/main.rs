use clap::Parser;
use env_logger;
use log::debug;
use sidebyside::Config;

fn setup_logging() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();
}

fn main() {
    let config = Config::parse();

    setup_logging();
    debug!("Comparing {} and {}...", config.file1, config.file2);
}
