use std::error::Error;

use aoc_rust::aoc_2022;

fn main() -> Result<(), Box<dyn Error>> {
    aoc_2022::day1::run();
    aoc_2022::day2::run();
    Ok(())
}
