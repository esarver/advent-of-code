#![feature(mpmc_channel)]

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("IO error: {0}")]
        IoError(#[from] std::io::Error),
    }
}

pub mod aoc {
    use std::{
        fmt::Display,
        fs::File,
        io::Read,
        path::PathBuf,
        sync::mpmc::{channel, Receiver, Sender},
        thread::{self, JoinHandle},
    };

    use crate::error::Error;

    type Solution = dyn Send + Sync + Fn(&[u8]) -> Result<u64, Error>;

    #[derive(Debug, Copy, Clone)]
    pub struct PartId {
        pub year: u16,
        pub day: u8,
        pub part: u8,
    }

    /// The prospective solution to a part of the Advent of Code
    #[derive(Clone)]
    pub struct Part {
        id: PartId,
        solution: &'static Solution,
    }

    /// The calculated answer to the Advent of Code part
    #[derive(Debug, Copy, Clone)]
    pub struct Answer {
        pub id: PartId,
        pub value: u64,
    }

    impl Display for Answer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    impl Part {
        pub fn new(year: u16, day: u8, part: u8, solution: &'static Solution) -> Self {
            Self {
                id: PartId { year, day, part },
                solution,
            }
        }

        pub fn run(&self, input: &[u8]) -> Result<Answer, Error> {
            Ok(Answer {
                id: self.id,
                value: (self.solution)(input)?,
            })
        }
    }

    pub struct Executor {
        jh: Vec<JoinHandle<Result<(), Error>>>,
        pub part_tx: Sender<Part>,
        pub ans_rx: Receiver<Result<Answer, Error>>,
    }

    impl Executor {
        pub fn new(jobs: u16, input_root_path: PathBuf) -> Result<Self, Error> {
            let (part_tx, part_rx) = channel::<Part>();
            let (ans_tx, ans_rx) = channel::<Result<Answer, Error>>();
            let mut jh = Vec::new();

            for i in 0..jobs {
                let input_root_path = input_root_path.clone();
                let part_rx = part_rx.clone();
                let ans_tx = ans_tx.clone();
                jh.push(
                    thread::Builder::new()
                        .name(format!("AoC_Job_{i}"))
                        .spawn(move || {
                            let part_rx = part_rx;
                            let ans_tx = ans_tx;
                            let input_root_path = input_root_path;

                            while let Ok(x) = part_rx.recv() {
                                let mut input: Vec<u8> = Vec::new();
                                let mut file = File::open(
                                    input_root_path
                                        .join(x.id.year.to_string())
                                        .join(x.id.day.to_string()),
                                )?;

                                let _ = file.read_to_end(&mut input);

                                let _ = ans_tx.send(x.run(&input));
                            }
                            Ok(())
                        })?,
                )
            }
            Ok(Self {
                jh,
                part_tx: part_tx.clone(),
                ans_rx,
            })
        }

        pub fn join(self) -> Result<(), Error> {
            for t in self.jh {
                match t.join() {
                    Ok(j) => j?,
                    Err(_) => continue,
                }
            }
            Ok(())
        }
    }
}

pub mod args {
    use clap::Parser;

    #[derive(Debug, Parser)]
    pub struct ArgParse {
        ///The list of years for which challenges should be run and reported. If no other options
        ///are provided, all parts of each day of the provided years will be run and reported.
        #[arg(short = 'y', long = "year")]
        years: Option<Vec<u8>>,

        ///The list of days for which challenges should be run and reported. If no year is provided, the
        ///latest year is assumed. If no part is provided, all parts will be run and reported.
        #[arg(short = 'd', long = "day")]
        days: Option<Vec<u8>>,

        /// The list of parts for which challenges should be run and reported.
        #[arg(short = 'p', long = "part")]
        parts: Option<Vec<u8>>,

        /// The number of jobs to run in parallel. Defaults to
        /// [`std::thread::available_parallelism`]
        #[arg(short = 'j', long = "jobs")]
        jobs: Option<u16>,
    }
}

use crate::{aoc::Part, args::ArgParse};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let _args = ArgParse::parse();

    println!(
        "1+1={}",
        Part::new(2024, 1, 1, &|input: &[u8]| Ok({
            let mut accum = 0u64;
            for i in input {
                accum += *i as u64;
            }
            accum
        }))
        .run(&[1, 1])?
    );

    Ok(())
}
