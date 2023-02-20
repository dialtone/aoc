use aoc;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of the AoC calendar
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..26))]
    day: u32,

    /// Year of the AoC
    #[arg(short, long, default_value = "2022")]
    year: u32,

    /// Part of the problem
    #[arg(short, long, value_parser=clap::value_parser!(u32).range(1..=2))]
    part: u32,
}

fn main() {
    let cli = Args::parse();

    let input = aoc::input::get_input(cli.year, cli.day).unwrap();

    aoc::solutions::get_solution(cli.year, cli.day, cli.part)(input.as_bytes());
}
