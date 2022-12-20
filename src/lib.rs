extern crate core;

use std::fs;
use std::time::{Duration, Instant};

mod day01;
mod day02;
mod day03;

pub fn run(day: i32) {
    match day {
        1 => {
            let input = read_input(day);

            let result = day01::part_one(input.clone());
            println!("day01::part_one() = {}", result);

            let result = day01::part_two(input.clone());
            println!("day01::part_two() = {}", result);
        }
        2 => {
            let input = read_input(day);

            let result = day02::part_one(input.clone());
            println!("day01::part_one() = {}", result);

            let result = day02::part_two(input.clone());
            println!("day01::part_two() = {}", result);
        }
        3 => {
            let input = read_input(day);

            let result = day03::part_one(input.clone());
            println!("day03::part_one() = {}", result);

            let result = day03::part_two(input.clone());
            println!("day03::part_two() = {}", result);
        }
        _ => println!("no puzzle for day {}", day),
    }
}

// # Utility

fn format_day_name(day: i32) -> String {
    if day < 10 {
        format!("Day0{}", day)
    } else {
        format!("Day{}", day)
    }
}

// read_file is a utility function that will read the input .txt file or panic
fn read_input(day: i32) -> String {
    let filename = if day < 10 {
        format!("day0{}", day)
    } else {
        format!("day{}", day)
    };
    fs::read_to_string(format!("input/{}.txt", filename)).expect("could not read file")
}

// measure is a utility function for measuring the execution time taken by the
// passed function
fn measure<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    Instant::now() - start
}
