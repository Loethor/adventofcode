use std::env;

mod common;
mod year_2021;

use common::inputs;

fn main() {

    let argv: Vec<String> = env::args().collect();
    if argv.len() != 3 {
        panic!("Usage: cargo run <year> <day>");
    }

    let year: u32 = argv[1].parse().expect("Year must be a number");
    let day: u8 = argv[2].parse().expect("Day must be a number");
    println!("Running year {}, day {}", year, day);

    match (year, day) {
        // 2021
        (2021, 1) => year_2021::day01::run(inputs::read_numbers(year, day)),
        (2021, 2) => year_2021::day02::run(inputs::read(year, day)),
        (2021, 3) => year_2021::day03::run(inputs::read(year, day)),
        (2021, 4) => year_2021::day04::run(inputs::read(year, day)),


        (_, _) => panic!("Not implemented :("),
    }
}
