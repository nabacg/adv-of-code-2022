use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[macro_use]
extern crate pest_derive;

fn read_lines(input_path: &str) -> Result<Vec<String>, std::io::Error> {
    let input_file = File::open(input_path)?;
    let lines = BufReader::new(input_file).lines();

    lines.collect()
}

pub fn print_result(aoc_day: &String, input_path: &String) -> Result<(), Box<dyn Error>> {
    match aoc_day.as_str() {
        "day1"  => day1::result(read_lines(input_path)?),
        "day2"  => day2::result(read_lines(input_path)?),
        "day3"  => day3::result(read_lines(input_path)?),
        "day4"  => day4::result(read_lines(input_path)?),
        "day5"  => day5::result(read_lines(input_path)?),
        "day6"  => day6::result(read_lines(input_path)?),
        "day7"  => day7::result(fs::read_to_string(input_path)?),
        "day8"  => day8::result(fs::read_to_string(input_path)?),
        "day9"  => day9::result(read_lines(input_path)?),
        "day10" => day10::result(read_lines(input_path)?),
        "day11" => day11::result(fs::read_to_string(input_path)?),        
        "day12" => day12::result(fs::read_to_string(input_path)?),
        "day13" => day13::result(read_lines(input_path)?),
        "day14" => day14::result(read_lines(input_path)?),
        "day15" => day15::result(read_lines(input_path)?),
        _d => Err(format!("Not implemented Advent Of Code Day selected: {}, currently only [day1,day2] are supported ", aoc_day).into()),        
    }
}
