use std::{fs::OpenOptions, path::PathBuf};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub fn init_logger(path: PathBuf) -> anyhow::Result<()> {
    let log = OpenOptions::new().append(true).create(true).open(path)?;

    let log = tracing_subscriber::fmt::layer()
        .with_writer(log)
        .with_ansi(false);

    let logger = Registry::default().with(LevelFilter::TRACE).with(log);

    tracing::subscriber::set_global_default(logger)?;

    Ok(())
}

