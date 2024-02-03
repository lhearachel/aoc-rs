mod aoc2022;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    year: u16,
    day: usize,
}

fn main() {
    let args = Args::parse();
    match args.year {
        2022 => match args.day {
            1 => aoc2022::day_1::solve(),
            _ => (),
        },
        _ => (),
    };
}

