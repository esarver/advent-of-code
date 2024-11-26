#![feature(mpmc_channel)]

pub mod aoc;
pub mod args;
pub mod error;
pub mod log;

pub mod years {
    pub mod _2023 {
        pub mod day01 {
            use crate::error::Error;

            pub fn part1(input: &[u8]) -> Result<i64, Error> {
                let mut accum: i64 = 0;
                for line in String::from_utf8(input.to_vec()).unwrap().lines() {
                    let numbers: Vec<char> = line.chars().filter(|e| e.is_numeric()).collect();
                    let (first, last) = (numbers.first().unwrap(), numbers.last().unwrap());
                    let number = (first.to_string().parse::<i64>().unwrap() * 10) + last.to_string().parse::<i64>().unwrap();
                    accum += number;
                }

                Ok(accum)

            }

            #[cfg(test)]
            mod unit {
                use super::part1;

                #[test]
                fn part1_test() {
                    let input = br"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
                    assert_eq!(part1(input).unwrap(), 142);
                }
            }
        }
    }
}

use crate::{aoc::Part, args::ArgParse, log::init_logger};
use aoc::Executor;
use clap::Parser;
use years::_2023;

fn main() -> anyhow::Result<()> {
    let args = ArgParse::parse();

    if let Some(l) = args.log {
        init_logger(l)?;
    }

    let input_root_path = args.input;

    let exec: Executor = Executor::new(std::thread::available_parallelism()?, input_root_path)?;

    let part_tx = exec.part_tx.clone();
    let ans_rx = exec.ans_rx.clone();

    let _2023_part1 = Part::new(
        2023,
        1,
        1,
        &_2023::day01::part1
    );

    part_tx.send(_2023_part1)?;

    while let Ok(ans) = ans_rx.recv()? {
        println!(
            "{}/{}-{}: {}",
            ans.id.year, ans.id.day, ans.id.part, ans.value
        );
    }

    exec.join()?;

    Ok(())
}
