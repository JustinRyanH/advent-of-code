use std::error;
use std::fs;
use std::io::prelude::Read;
use std::path::PathBuf;

use clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum AdventDay {
        Day1,
        Day2,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "advent of code",
    about = "Main Runner for Advent of Code Solution"
)]
pub struct Opts {
    #[structopt(possible_values = &AdventDay::variants(), case_insensitive = true)]
    day: AdventDay,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opts::from_args();
    let mut file = fs::File::open(opt.path)?;
    match opt.day {
        AdventDay::Day1 => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            let (numbers, errors): (Vec<_>, Vec<_>) = buffer
                .split("\n")
                .into_iter()
                .enumerate()
                .map(|(line, value)| match value.parse::<u32>() {
                    Ok(v) => Ok(v),
                    Err(_) => Err(format!("Could not parse {:?} on line: {:?}", value, line)),
                })
                .partition(Result::is_ok);
            if let Some(err) = errors.into_iter().nth(0) {
                err.unwrap();
            }
            let numbers: Vec<u32> = numbers.into_iter().map(|a| a.unwrap()).collect();
            let result = day1::part1(&numbers);
            match result {
                Some(r) => println!("Day1: Your 2020 Expenses are {}", r),
                None => println!("Day1: Something went wrong, you don't have 2020 Expense"),
            }
        }
        _ => println!("Unimplemented Day"),
    }
    Ok(())
}
