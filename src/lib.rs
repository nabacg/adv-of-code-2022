use std::{fs::{self, File}, io::{BufReader, BufRead}, error::Error};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

extern crate pest;
#[macro_use]
extern crate pest_derive;


fn read_lines(input_path: &str) -> Result<Vec<String>, std::io::Error> {
    let input_file = File::open(input_path)?;
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
        _d => 
            Err(format!("Not implemented Advent Of Code Day selected: {}, currently only [day1,day2] are supported ", aoc_day).into())
            
        
    }
}