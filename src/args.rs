use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct ArgParse {
    ///The list of years for which challenges should be run and reported. If no other options
    ///are provided, all parts of each day of the provided years will be run and reported.
    #[arg(short = 'y', long = "year")]
    pub years: Option<Vec<u8>>,

    ///The list of days for which challenges should be run and reported. If no year is provided, the
    ///latest year is assumed. If no part is provided, all parts will be run and reported.
    #[arg(short = 'd', long = "day")]
    pub days: Option<Vec<u8>>,

    /// The list of parts for which challenges should be run and reported.
    #[arg(short = 'p', long = "part")]
    pub parts: Option<Vec<u8>>,

    /// The number of jobs to run in parallel. Defaults to
    /// [`std::thread::available_parallelism`]
    #[arg(short = 'j', long = "jobs")]
    pub jobs: Option<u16>,

    /// The root path of the inputs.
    #[arg(short = 'i', long = "input")]
    pub input: PathBuf,

    /// The file in which to write logs
    #[arg(short = 'l', long = "log")]
    pub log: Option<PathBuf>,
}
