use aoc_2024::AocDay;
use std::env;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let days = AocDay::all_days();
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        return Err(Error::new(ErrorKind::InvalidInput, "Not enough arguments").into());
    }

    days[args[1].parse::<usize>()? - 1].solve(&args[2]);
    Ok(())
}
