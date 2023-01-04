
use std::{
    env,
    error::Error,
    process,
};



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("2 cmd argument required:\n - day of Advent of Code puzzle, in day1, day2,etc format\n - path to the input text file");
        process::exit(1);
    }
    let aoc_day = &args[1];
    let input_path = &args[2];

    adv_of_code_2022::print_result(aoc_day, input_path)
}
