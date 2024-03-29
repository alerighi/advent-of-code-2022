mod problem;
mod days;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

use problem::AoCProblem;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// day to solve
    day: u32,

    /// solve example code
    #[arg(short, long)]
    example: bool,

    /// generate template for the day
    #[arg(short, long)]
    template: bool,

    /// dump the input
    #[arg(short, long)]
    dump_input: bool
}

fn main() {
    let args = Args::parse();

    if args.template {
        problem::create_template(args.day).unwrap();
        return
    }

    println!("*** solving day {} ***", args.day);

    let mut problem: Box<dyn AoCProblem> = match args.day {
        1 => Box::new(days::day01::AoCDay1::default()),
        2 => Box::new(days::day02::AoCDay2::default()),
        3 => Box::new(days::day03::AoCDay3::default()),
        4 => Box::new(days::day04::AoCDay4::default()),
        5 => Box::new(days::day05::AoCDay5::default()),
        6 => Box::new(days::day06::AoCDay6::default()),
        7 => Box::new(days::day07::AoCDay7::default()),
        8 => Box::new(days::day08::AoCDay8::default()),
        9 => Box::new(days::day09::AoCDay9::default()),
        10 => Box::new(days::day10::AoCDay10::default()),
        11 => Box::new(days::day11::AoCDay11::default()),
        12 => Box::new(days::day12::AoCDay12::default()),
        13 => Box::new(days::day13::AoCDay13::default()),
        14 => Box::new(days::day14::AoCDay14::default()),
        15 => Box::new(days::day15::AoCDay15::default()),
        _ => panic!("day not yet implemented"),
    };

    let input_path = if args.example {
        format!("input/{:02}/example.txt", args.day)
    } else {
        format!("input/{:02}/input.txt", args.day)
    };

    let file = File::open(input_path)
        .expect("error opening file");
    let buffer_reader = BufReader::new(file);
    for line in buffer_reader.lines() {
        match line {
            Ok(line) => problem.parse_line(line),
            Err(_) => break,
        }
    }

    if args.dump_input {
        println!("PARSED INPUT: {:#?}", problem);
    }

    let time_1 = Instant::now();
    let result_1 = problem.solve_part1();
    println!("DAY{} PART 1 solution = {} ({}ms)", args.day, result_1, time_1.elapsed().as_millis());

    let time_2 = Instant::now();
    let result_2 = problem.solve_part2();
    println!("DAY{} PART 2 solution = {} ({}ms)", args.day, result_2, time_2.elapsed().as_millis());
}
