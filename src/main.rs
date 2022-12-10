mod days;
mod problem;
use crate::days::prelude::*;
use clap::Parser;
use problem::Problem;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day
    day: usize,
}

fn main() {
    let args = Cli::parse();
    let day: usize = args.day;
    let day = day_to_problem(day);
    println!("{}", day.as_ref().unwrap().part_one());
    println!("{}", day.unwrap().part_two());
}

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne {})),
        2 => Some(Box::new(DayTwo {})),
        3 => Some(Box::new(DayThree {})),
        4 => Some(Box::new(DayFour {})),
        _ => None,
    }
}
