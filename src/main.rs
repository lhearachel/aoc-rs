mod aoc2022;
mod io;

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
            2 => aoc2022::day_2::solve(),
            v => println!("Unsupported value day={v}"),
        },
        v => println!("Unsupported value year={v}"),
    };
}

