mod day01;

use clap::Parser;
use std::{str::FromStr, fmt::Display};

#[derive(Parser,Debug)]
enum Args {
    /// Calorie Counting
    ///
    /// Summing groups of numbers.
    Day1(Opts)
}
use Args::*;

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
enum Star {
    One,
    Two
}

impl FromStr for Star {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "one" => Ok(Self::One),
            "2" | "two" => Ok(Self::Two),
            _ => anyhow::bail!("Expecting '1' or '2'")
        }
    }
}

#[derive(Parser,Debug)]
struct Opts {
    #[clap(long, short)]
    file: File,
    #[clap(long, short)]
    star: Star,
}

#[derive(Debug)]
pub struct File {
    pub contents: String
}

impl FromStr for File {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let contents = std::fs::read_to_string(s)?;
        Ok(File { contents })
    }
}

trait PrintStar {
    fn print_star(&self);
}
impl <T: Display, E: std::fmt::Debug> PrintStar for Result<T,E> {
    fn print_star(&self) {
        match self {
            Ok(res) => println!("{res}"),
            Err(e) => eprintln!("Error: {e:?}")
        }
    }
}

fn main() {
    let args = Args::parse();

    let res: Box<dyn PrintStar> = Box::new(match args {
        Day1(Opts { star: Star::One, file }) => day01::star1(file),
        Day1(Opts { star: Star::Two, file }) => day01::star2(file)
    });

    res.print_star();
}
