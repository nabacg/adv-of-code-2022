use std::{fs::{self}, io::{BufReader, BufRead}, env, error::Error, process};

struct Elf {
    calories: i32,
}

struct ElfExpedition {
    elves: Vec<Elf>,
    elf_candidate: Vec<i32>,
}

impl ElfExpedition {
    pub fn new() -> ElfExpedition {
        return ElfExpedition { elves: vec![], elf_candidate: vec![] }
    }

    fn pack_snack(&mut self, s: i32) {
        self.elf_candidate.push(s);
    }

    fn pack_elf(&mut self) {
        if self.elf_candidate.is_empty() {
            return;
        }
        let calorie_total = self.elf_candidate.iter().sum();
        self.elves.push(Elf{calories: calorie_total});
        self.elf_candidate.clear();
    }

    fn top_three_total(&mut self) -> i32 {
        self.elves.sort_by(|a,b| b.calories.cmp(&a.calories));
        self.elves.iter().take(3).map(|e| e.calories).sum()
    }
}


fn parse_elves(lines: Vec<String>) -> Result<ElfExpedition, Box<dyn Error>> {
    let mut elves = lines.iter().fold(ElfExpedition::new(), |mut acc, l| {
        match l {
            l if l.len() == 0 => { 
                acc.pack_elf();
                acc
            },
            l => {
                let cals = l.parse::<i32>().unwrap();
                acc.pack_snack(cals);
                acc
            },
        }

    });
    //collect the last elf
    elves.pack_elf();
    Ok(elves)
}


fn read_lines2(input_path: &str) -> Result<Vec<String>, std::io::Error> {
    let input_file = fs::File::open(input_path)?;
    let lines = BufReader::new(input_file).lines()
    .fold(Ok(vec![]), |acc, l|{
        match (acc, l) {
            (Ok(mut acc), Ok(l)) => {
                acc.push(l);
                Ok(acc)
            },
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e), Err(_)) => Err(e), // let's say the first error wins
            
        }
    });
    lines
}

fn read_lines(input_path: &str) -> Result<Vec<String>, std::io::Error> {
    let input_file = fs::File::open(input_path)?;
    let lines = BufReader::new(input_file).lines();
    
    lines.collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_path =    "test_input.txt";
    // if args.len() < 2 {
    //     eprintln!("1 cmd argument required: provide path to the input file");
    //     process::exit(1);
    // }
    // let input_path = &args[1];

    let lines = read_lines(input_path)?;
    let mut elves = parse_elves(lines)?;


    println!("result is: {}", elves.top_three_total());
    Ok(())

}
