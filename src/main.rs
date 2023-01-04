
use std::{
    env,
    error::Error,
    fs::{self},
    io::{BufRead, BufReader},
    process,
};
use adv_of_code_2022::{day1, day2, day3, day4, day5, day6, day7};




fn read_lines(input_path: &str) -> Result<Vec<String>, std::io::Error> {
    let input_file = fs::File::open(input_path)?;
    let lines = BufReader::new(input_file).lines();

    lines.collect()
}

pub fn print_result(aoc_day: &String, input_path: &String) -> Result<(), Box<dyn Error>> {
    let lines = read_lines(input_path)?;

    match aoc_day.as_str() {
        "day1" => day1::result(lines),
        "day2" => day2::result(lines),
        "day3" => day3::result(lines),
        "day4" => day4::result(lines),
        "day5" => day5::result(lines),
        "day6" => day6::result(lines),
        "day7" => day7::result(lines),
        _d => {
            eprintln!("Not implemented Advent Of Code Day selected: {}, currently only [day1,day2] are supported ", aoc_day);
            process::exit(1)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("2 cmd argument required:\n - day of Advent of Code puzzle, in day1, day2,etc format\n - path to the input text file");
        process::exit(1);
    }
    let aoc_day = &args[1];
    let input_path = &args[2];

    print_result(aoc_day, input_path)
}
