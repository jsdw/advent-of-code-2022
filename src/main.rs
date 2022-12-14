#[macro_use] mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

use clap::Parser;
use std::{str::FromStr, fmt::Display};

#[derive(Parser,Debug)]
enum Args {
    /// Calorie Counting
    ///
    /// Summing groups of numbers and then summing best 3 groups.
    Day1(Opts),
    /// Rock Paper Scissors
    ///
    /// Using set moves to win.
    Day2(Opts),
    /// Rucksack Reorganization
    ///
    /// Find the duplicate letter in each half of a string (packing items into compartments),
    /// and then in each group of 3 strings.
    Day3(Opts),
    /// Camp Cleanup
    ///
    /// Find overlapping work schedules.
    Day4(Opts),
    /// Supply Stacks
    ///
    /// Rearranging crates stacked on top of eachother by following instructions.
    Day5(Opts),
    /// Tuning Trouble
    ///
    /// Find first 4 or 14 non-repeating letters.
    Day6(Opts),
    /// No Space Left On Device
    ///
    /// Parse terminal commands to worrk out file structure and sizes of things.
    Day7(Opts),
    /// Treetop Tree House
    ///
    /// Counting visible trees in a grid based on their height.
    Day8(Opts),
    /// Rope Bridge
    ///
    /// A rope tail following a head around.
    Day9(Opts),
    /// Cathode-Ray Tube
    ///
    /// Basic assembly; noop & addx. Check value of X at cycles. Print a message.
    Day10(Opts),
    /// Monkey in the Middle
    ///
    /// Monkeys throwing items to eachother with rules. Hardcoded, so no file input.
    /// Couldn't be bothered to parse it. Liked the part 2.
    Day11{ #[clap(short,long)] star: Star },
    /// Hill Climbing Algorithm
    ///
    /// Find the shortest path(s) up a hill.
    Day12(Opts),
    /// Distress Signal
    ///
    /// Compare pairs of items/lists of items, and then sort them.
    Day13(Opts),
    /// Regolith Reservoir
    ///
    /// Simulate falling sand and see how much settles.
    Day14(Opts),
    /// Beacon Exclusion Zone
    ///
    /// Sensors and beacons; which square is the actual beacon on?
    Day15(Opts),
    /// Proboscidea Volcanium
    ///
    /// Searching tunnels and release valves to find best order to release most pressure.
    Day16(Opts),
}

fn main() {
    use Args::*;
    let args = Args::parse();

    match args {
        Day1(Opts { star: Star::One, file }) => print(day01::star1(file)),
        Day1(Opts { star: Star::Two, file }) => print(day01::star2(file)),
        Day2(Opts { star: Star::One, file }) => print(day02::star1(file)),
        Day2(Opts { star: Star::Two, file }) => print(day02::star2(file)),
        Day3(Opts { star: Star::One, file }) => print(day03::star1(file)),
        Day3(Opts { star: Star::Two, file }) => print(day03::star2(file)),
        Day4(Opts { star: Star::One, file }) => print(day04::star1(file)),
        Day4(Opts { star: Star::Two, file }) => print(day04::star2(file)),
        Day5(Opts { star: Star::One, file }) => print(day05::star1(file)),
        Day5(Opts { star: Star::Two, file }) => print(day05::star2(file)),
        Day6(Opts { star: Star::One, file }) => print(day06::star1(file)),
        Day6(Opts { star: Star::Two, file }) => print(day06::star2(file)),
        Day7(Opts { star: Star::One, file }) => print(day07::star1(file)),
        Day7(Opts { star: Star::Two, file }) => print(day07::star2(file)),
        Day8(Opts { star: Star::One, file }) => print(day08::star1(file)),
        Day8(Opts { star: Star::Two, file }) => print(day08::star2(file)),
        Day9(Opts { star: Star::One, file }) => print(day09::star1(file)),
        Day9(Opts { star: Star::Two, file }) => print(day09::star2(file)),
        Day10(Opts { star: Star::One, file }) => print(day10::star1(file)),
        Day10(Opts { star: Star::Two, file }) => print(day10::star2(file)),
        Day11{ star: Star::One } => print(day11::star1()),
        Day11{ star: Star::Two } => print(day11::star2()),
        Day12(Opts { star: Star::One, file }) => print(day12::star1(file)),
        Day12(Opts { star: Star::Two, file }) => print(day12::star2(file)),
        Day13(Opts { star: Star::One, file }) => print(day13::star1(file)),
        Day13(Opts { star: Star::Two, file }) => print(day13::star2(file)),
        Day14(Opts { star: Star::One, file }) => print(day14::star1(file)),
        Day14(Opts { star: Star::Two, file }) => print(day14::star2(file)),
        Day15(Opts { star: Star::One, file }) => print(day15::star1(file)),
        Day15(Opts { star: Star::Two, file }) => print(day15::star2(file)),
        Day16(Opts { star: Star::One, file }) => print(day16::star1(file)),
        Day16(Opts { star: Star::Two, file }) => print(day16::star2(file)),
    }
}

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

fn print<T: Display, E: std::fmt::Debug>(val: Result<T, E>) {
    match val {
        Ok(res) => println!("{res}"),
        Err(e) => eprintln!("Error: {e:?}")
    }
}
