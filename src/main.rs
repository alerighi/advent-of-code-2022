mod problem;
mod days;

use std::fs::File;
use std::io::{BufReader, BufRead};

use problem::AoCProblem;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: u32 = args[1].parse().unwrap();
    let is_example = args.len() > 2 && args[2] == "ex";
    let is_template = args.len() > 2 && args[2] == "t";

    if is_template {
        problem::create_template(day).unwrap();
        return
    }

    println!("*** solving day {} ***", day);

    let mut problem: Box<dyn AoCProblem> = match day {
        1 => Box::new(days::day01::AoCDay1::default()),
        2 => Box::new(days::day02::AoCDay2::default()),
        3 => Box::new(days::day03::AoCDay3::default()),
        4 => Box::new(days::day04::AoCDay4::default()),
        5 => Box::new(days::day05::AoCDay5::default()),
        6 => Box::new(days::day06::AoCDay6::default()),
        _ => panic!("day not yet implemented"),
    };

    let input_path = if is_example {
        format!("input/{:02}/example.txt", day)
    } else {
        format!("input/{:02}/input.txt", day)
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

    println!("DAY{} PART 1 solution = {}", day, problem.solve_part1());
    println!("DAY{} PART 2 solution = {}", day, problem.solve_part2());
}
