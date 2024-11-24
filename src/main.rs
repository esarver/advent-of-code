#![feature(mpmc_channel)]

pub mod aoc;
pub mod args;
pub mod error;
pub mod log;

use crate::{aoc::Part, args::ArgParse, log::init_logger};
use aoc::Executor;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = ArgParse::parse();

    if let Some(l) = args.log {
        init_logger(l)?;
    }

    let input_root_path = args.input;

    let exec: Executor = Executor::new(std::thread::available_parallelism()?, input_root_path)?;

    let part_tx = exec.part_tx.clone();
    let ans_rx = exec.ans_rx.clone();

    let part0 = Part::new(2023, 1, 1, &|input| {
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
