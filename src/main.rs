use std::{fs::{self, File}, io::{BufReader, BufRead, Lines}, env, process, error::Error};

struct ElfExpedition {
    max_calories: i32,
    current_sum: i32,
}

fn read_lines(input_path: &str) -> Result<Lines<BufReader<File>>, Box<dyn Error>> {
    let input_file = fs::File::open(input_path)?;
    let lines = BufReader::new(input_file).lines();
    return Ok(lines)
}

fn parse_elves(lines: Lines<BufReader<File>>) -> Result<ElfExpedition, Box<dyn Error>> {
    let elves = lines.fold(ElfExpedition{max_calories: 0,  current_sum: 0}, |acc, l| {
        match l {
            Ok(l) if l.len() == 0 => if acc.current_sum > acc.max_calories {
                ElfExpedition{max_calories: acc.current_sum, current_sum: 0}
            } else {
                ElfExpedition{max_calories: acc.max_calories, current_sum: 0}
            },
            Ok(l) => {
                let cals = l.parse::<i32>().unwrap();
                ElfExpedition{max_calories: acc.max_calories, current_sum: acc.current_sum + cals}
            },
            Err(msg) => panic!("Unexpected error when reading input lines")
        }

    });
    Ok(elves)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("1 cmd argument required: provide path to the input file");
        process::exit(1);
    }
    let input_path = &args[1];

    let lines = read_lines(input_path)?;
    let elves = parse_elves(lines)?;


    println!("result is: {}", elves.max_calories);
    Ok(())

}
