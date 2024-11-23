#![feature(mpmc_channel)]

pub mod args;
pub mod error;

pub mod aoc {
    use std::{
        fmt::Display,
        fs::File,
        io::Read,
        num::NonZero,
        path::PathBuf,
        sync::mpmc::{channel, Receiver, Sender},
        thread::{self, JoinHandle},
    };

    use crate::error::Error;

    type Solution = dyn Send + Sync + Fn(&[u8]) -> Result<i64, Error>;

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
        pub value: i64,
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
        pub part_tx: Sender<Part>,
        pub ans_rx: Receiver<Result<Answer, Error>>,
        jh: Vec<JoinHandle<Result<(), Error>>>,
    }

    impl Executor {
        pub fn new(jobs: NonZero<usize>, input_root_path: PathBuf) -> Result<Self, Error> {
            let (part_tx, part_rx) = channel::<Part>();
            let (ans_tx, ans_rx) = channel::<Result<Answer, Error>>();
            let mut jh = Vec::new();

            for i in 0..jobs.into() {
                println!("Creating worker {i}");
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

                                println!("{}: Received {}/{}-{}", thread_name, x.id.year, x.id.day, x.id.part);
                                let mut input: Vec<u8> = Vec::new();
                                let path = input_root_path
                                        .join(x.id.year.to_string())
                                        .join(x.id.day.to_string());

                                println!("{}: Opening file {}", thread_name, path.as_path().display());
                                let mut file = match File::open(
                                    &path
                                ) {
                                    Ok(f) => f,
                                    Err(e) => {
                                        println!("{}: Error - {}", thread_name, e);
                                        return Err(e.into());
                                    }
                                };

                                println!("{}: Reading file {}", thread_name, path.as_path().display());

                                match file.read_to_end(&mut input) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        println!("{}: Error - {}", thread_name, e);
                                        return Err(e.into());
                                    }
                                };


                                println!("{}: Running {}/{}-{}", thread_name, x.id.year, x.id.day, x.id.part);
                                let ans = x.run(&input);
                                println!("{}: Answer {}/{}-{} => {}", thread_name, x.id.year, x.id.day, x.id.part, ans?.value);
                                match ans_tx.send(x.run(&input)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        println!("{}: Error - {}", thread_name, e);
                                        return Err(Error::SendChannelError(e.to_string()));
                                    }
                                };
                                println!("{}: Completed {}/{}-{}", thread_name, x.id.year, x.id.day, x.id.part);
                            }
                           println!("{}: Closing", thread_name);
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
        use crate::aoc::Part;
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
                2
            );

            Ok(())
        }
    }
}

use crate::{aoc::Part, args::ArgParse, error::Error};
use aoc::Executor;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = ArgParse::parse();

    let input_root_path = args.input;

    let exec: Executor = Executor::new(std::thread::available_parallelism()?, input_root_path)?;

    let part_tx = exec.part_tx.clone();
    let ans_rx = exec.ans_rx.clone();

    let part0 = Part::new(2023, 1, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });
    let part1 = Part::new(2023, 2, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });
    let part2 = Part::new(2023, 3, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });
    let part3 = Part::new(2023, 4, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });
    let part4 = Part::new(2023, 5, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });
    let part5 = Part::new(2023, 6, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });
    let part6 = Part::new(2023, 7, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });
    let part7 = Part::new(2023, 8, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });
    let part8 = Part::new(2023, 9, 1, &|input| {
        println!("Input: {input:?}");
        let mut accum = 0i64;
        for x in input {
            accum += match x {
                b'+' => 1,
                b'-' => -1,
                b'\n' => 0,
                _ => 0,
            };
        }
        Ok(accum)
    });

    part_tx.send(part0)?;
    part_tx.send(part1)?;
    part_tx.send(part2)?;
    part_tx.send(part3)?;
    part_tx.send(part4)?;
    part_tx.send(part5)?;
    part_tx.send(part6)?;
    part_tx.send(part7)?;
    part_tx.send(part8)?;

    while let Ok(ans) = ans_rx.recv()? {
        println!(
            "{}/{}-{}: {}",
            ans.id.year, ans.id.day, ans.id.part, ans.value
        );
    }

    exec.join()?;

    Ok(())
}
