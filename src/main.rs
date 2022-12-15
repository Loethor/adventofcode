use std::env;

mod common;
mod year_2021;
mod year_2022;

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
        (2021,  1) => year_2021::day01::run(inputs::read_numbers(year, day)),
        (2021,  2) => year_2021::day02::run(inputs::read(year, day)),
        (2021,  3) => year_2021::day03::run(inputs::read(year, day)),
        (2021,  4) => year_2021::day04::run(inputs::read(year, day)),
        (2021,  5) => year_2021::day05::run(inputs::read(year, day)),

        (2021,  6) => year_2021::day06::run(inputs::read(year, day)),
        (2021,  7) => year_2021::day07::run(inputs::read(year, day)),
        (2021,  8) => year_2021::day08::run(inputs::read(year, day)),
        (2021,  9) => year_2021::day09::run(inputs::read(year, day)),
        (2021, 10) => year_2021::day10::run(inputs::read(year, day)),

        (2021, 11) => year_2021::day11::run(inputs::read(year, day)),
        (2021, 12) => year_2021::day12::run(inputs::read(year, day)),
        (2021, 13) => year_2021::day13::run(inputs::read(year, day)),
        (2021, 14) => year_2021::day14::run(inputs::read(year, day)),


        //2022
        (2022,  1) => year_2022::day01::run(inputs::read(year, day)),
        (2022,  2) => year_2022::day02::run(inputs::read(year, day)),
        (2022,  3) => year_2022::day03::run(inputs::read(year, day)),
        (2022,  4) => year_2022::day04::run(inputs::read(year, day)),
        (2022,  5) => year_2022::day05::run(inputs::read(year, day)),

        (2022,  6) => year_2022::day06::run(inputs::read(year, day)),
        (2022,  7) => year_2022::day07::run(inputs::read(year, day)),
        (2022,  8) => year_2022::day08::run(inputs::read(year, day)),



        (_, _) => panic!("Solution for Year: {} Day {} is not implemented :(", year, day),
    }
}
