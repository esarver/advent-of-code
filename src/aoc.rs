use std::{
    fmt::Display,
    fs::File,
    io::Read,
    num::NonZero,
    path::PathBuf,
    sync::mpmc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use tracing::{debug, error, info, instrument, trace, warn, warn_span};

use crate::error::Error;

type Solution = dyn Send + Sync + Fn(&[u8]) -> Result<i64, Error>;

#[derive(Debug, Copy, Clone, Hash)]
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
    pub value: Status,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    NotRegistered,
    Waiting,
    ReadingInput,
    Running,
    Completed(i64),
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NotRegistered => "Not Registered".to_string(),
                Self::Waiting => "Waiting".to_string(),
                Self::ReadingInput => "Reading Input".to_string(),
                Self::Running => "Running".to_string(),
                Self::Completed(v) => v.to_string(),
            }
        )
    }
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
            value: Status::Completed((self.solution)(input)?),
        })
    }
}

pub struct Executor {
    pub part_tx: Sender<Part>,
    pub ans_rx: Receiver<Result<Answer, Error>>,
    jh: Vec<JoinHandle<Result<(), Error>>>,
}

impl Executor {
    #[instrument]
    pub fn new(jobs: NonZero<usize>, input_root_path: PathBuf) -> Result<Self, Error> {
        let (part_tx, part_rx) = channel::<Part>();
        let (ans_tx, ans_rx) = channel::<Result<Answer, Error>>();
        let mut jh = Vec::new();

        for i in 0..jobs.into() {
            info!("Creating worker {i}");
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
                        let thread_name = std::thread::current();
                        let thread_name = thread_name.name().unwrap_or_default();

                        while let Ok(x) = part_rx.recv() {
                            debug!(
                                "{}: Received {}/{}-{}",
                                thread_name, x.id.year, x.id.day, x.id.part
                            );
                            match ans_tx.send(Ok(Answer{id: x.id, value: Status::Running})) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("{}: Error - {}", thread_name, e);
                                    return Err(Error::SendChannelError(e.to_string()));
                                }
                            };
                            let mut input: Vec<u8> = Vec::new();
                            let path = input_root_path
                                .join(x.id.year.to_string())
                                .join(x.id.day.to_string());

                            debug!("{}: Opening file {}", thread_name, path.as_path().display());
                            match ans_tx.send(Ok(Answer{id: x.id, value: Status::ReadingInput})) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("{}: Error - {}", thread_name, e);
                                    return Err(Error::SendChannelError(e.to_string()));
                                }
                            };
                            let mut file = match File::open(&path) {
                                Ok(f) => f,
                                Err(e) => {
                                    error!("{}: Error - {}", thread_name, e);
                                    return Err(e.into());
                                }
                            };

                            debug!("{}: Reading file {}", thread_name, path.as_path().display());

                            match file.read_to_end(&mut input) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("{}: Error - {}", thread_name, e);
                                    return Err(e.into());
                                }
                            };

                            debug!(
                                "{}: Running {}/{}-{}",
                                thread_name, x.id.year, x.id.day, x.id.part
                            );
                            match ans_tx.send(Ok(Answer{id: x.id, value: Status::Running})) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("{}: Error - {}", thread_name, e);
                                    return Err(Error::SendChannelError(e.to_string()));
                                }
                            };
                            let ans = x.run(&input);
                            info!(
                                "{}: Answer {}/{}-{} => {}",
                                thread_name, x.id.year, x.id.day, x.id.part, ans?.value
                            );
                            match ans_tx.send(x.run(&input)) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("{}: Error - {}", thread_name, e);
                                    return Err(Error::SendChannelError(e.to_string()));
                                }
                            };
                            debug!(
                                "{}: Completed {}/{}-{}",
                                thread_name, x.id.year, x.id.day, x.id.part
                            );
                        }
                        warn!("{}: Closing", thread_name);
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

#[cfg(test)]
mod unit {
    use crate::aoc::{Part, Status};
    #[test]
    fn part_run_functions() -> anyhow::Result<()> {
        assert_eq!(
            Part::new(2024, 1, 1, &|input: &[u8]| Ok({
                let mut accum = 0i64;
                for i in input {
                    accum += *i as i64;
                }
                accum
            }))
            .run(&[1, 1])?
            .value,
            Status::Completed(2),
        );

        Ok(())
    }
}
