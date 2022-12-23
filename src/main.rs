use std::{
    env,
    error::Error,
    fs::{self},
    io::{BufRead, BufReader},
    process, collections::HashSet,
};

// DAY 1
struct Elf {
    calories: i32,
}

struct ElfExpedition {
    elves: Vec<Elf>,
    elf_candidate: Vec<i32>,
}

impl ElfExpedition {
    pub fn new() -> ElfExpedition {
        return ElfExpedition {
            elves: vec![],
            elf_candidate: vec![],
        };
    }

    fn pack_snack(&mut self, s: i32) {
        self.elf_candidate.push(s);
    }

    fn pack_elf(&mut self) {
        if self.elf_candidate.is_empty() {
            return;
        }
        let calorie_total = self.elf_candidate.iter().sum();
        self.elves.push(Elf {
            calories: calorie_total,
        });
        self.elf_candidate.clear();
    }

    fn top_three_total(&mut self) -> i32 {
        self.elves.sort_by(|a, b| b.calories.cmp(&a.calories));
        self.elves.iter().take(3).map(|e| e.calories).sum()
    }
}

fn parse_elves(lines: Vec<String>) -> Result<ElfExpedition, Box<dyn Error>> {
    let mut elves = lines
        .iter()
        .fold(ElfExpedition::new(), |mut acc, l| match l {
            l if l.len() == 0 => {
                acc.pack_elf();
                acc
            }
            l => {
                let cals = l.parse::<i32>().unwrap();
                acc.pack_snack(cals);
                acc
            }
        });
    //collect the last elf
    elves.pack_elf();
    Ok(elves)
}

fn day1_result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut elves = parse_elves(lines)?;

    println!("result is: {}", elves.top_three_total());
    Ok(())
}

// DAY 1 END

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_plain_code(c: &str) -> Result<Shape, &str> {
        match c {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err("invalid plain code, only A, B and C are supported!"),
        }
    }

    pub fn from_secret_code(c: &str) -> Result<Shape, &str> {
        match c {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err("invalid secret code, only X, Y and Z are supported!"),
        }
    }

    pub fn from_expected_result(r: &GameResult, opponent_move: &Shape) -> Result<Shape, String> {
        let simulated_game = vec![Shape::Rock, Shape::Paper, Shape::Scissors]
            .iter()
            .map(|p2| Game {
                player_1: *opponent_move,
                player_2: *p2,
            })
            .filter(|g| &g.player_2_result() == r)
            .take(1)
            .next();
        if let Some(matching_game) = simulated_game {
            Ok(matching_game.player_2)
        } else {
            Err("Couldn't simulate a expected move to match oppenent_move and produce expected game_result".to_string())
            //, opponent_move, r).as_str())
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GameResult {
    Lose,
    Draw,
    Win,
}

impl GameResult {
    pub fn score(&self) -> i32 {
        match self {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }

    pub fn from_secret_code(c: &str) -> Result<GameResult, &str> {
        match c {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err("invalid secret result code, only X, Y and Z are supported!"),
        }
    }
}

pub struct Game {
    player_1: Shape,
    player_2: Shape,
}

impl Game {
    pub fn from_input(line: &str) -> Result<Game, String> {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        match parts[..] {
            [left, right] => {
                let l_hand =   Shape::from_plain_code(left)?;
                // let r_hand = Shape::from_secret_code(right)?;
                let expected_result = GameResult::from_secret_code(right)?;
                let r_hand = Shape::from_expected_result(&expected_result, &l_hand)?;
                Ok(Game{
                    player_1: l_hand,
                    player_2: r_hand,
                })
            },
            _ => Err(format!("Invalid inputs found, expected 2 whitespace separated single letter codes, got: {}", line))
        }
    }
    pub fn player_2_result(&self) -> GameResult {
        match (&self.player_1, &self.player_2) {
            (Shape::Rock, Shape::Paper) => GameResult::Win,
            (Shape::Paper, Shape::Scissors) => GameResult::Win,
            (Shape::Scissors, Shape::Rock) => GameResult::Win,
            (Shape::Rock, Shape::Rock) => GameResult::Draw,
            (Shape::Paper, Shape::Paper) => GameResult::Draw,
            (Shape::Scissors, Shape::Scissors) => GameResult::Draw,
            (Shape::Rock, Shape::Scissors) => GameResult::Lose,
            (Shape::Scissors, Shape::Paper) => GameResult::Lose,
            (Shape::Paper, Shape::Rock) => GameResult::Lose,
        }
    }

    pub fn player_2_score(&self) -> i32 {
        self.player_2_result().score() + self.player_2.score()
    }
}

fn day2_result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let games: Result<Vec<Game>, String> = lines.iter().map(|l| Game::from_input(l)).collect();
    match games {
        Ok(games) => {
            let scores: i32 = games.iter().map(|g| g.player_2_score()).sum();
            println!("result is: {}", scores);
        }
        Err(e) => {
            eprintln!("error processing input: {}", e);
        }
    }
    Ok(())
}
// DAY 2

// DAY 3 

pub struct Rucksack {
    compartment_a: Vec<char>,
    compartment_b: Vec<char>,
}

impl Rucksack {
pub fn from_input(l: &String) -> Result<Rucksack, &str> {
        if l.chars().count() % 2 != 0 {
            return Err("Invalid input, only even character counts are supported")
        }
        let (a, b) =    l.split_at(l.chars().count()/2);
        Ok(Rucksack { compartment_a: a.chars().collect(), compartment_b: b.chars().collect() })
    }

    pub fn overlaps(&self) -> Vec<&char> {
        let comp_a_set= HashSet::<_>::from_iter(self.compartment_a.iter());
        let dedup = HashSet::<_>::from_iter(self.compartment_b.iter().filter(|b| comp_a_set.contains(b)));
        dedup.iter().map(|c| *c).collect()
    }

    pub fn item_priority(i: &char) -> u32 {
        // println!("i: {}",i);
        let ascii = *i as u32;
        if i.is_uppercase() {
            ascii - 38 // - 65 + 26
        } else {
            ascii - 96
        }
  
    }
}

fn day3_result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let rucksacks: Result<Vec<Rucksack>, &str> = lines.iter().map(Rucksack::from_input).collect();
    match rucksacks {
        Ok(rucksacks) => { 
            let misplaced_items: u32 = rucksacks.iter()
                .flat_map(|r| r.overlaps())
                .map(Rucksack::item_priority)
                .sum();
            println!("result is {}", misplaced_items);
        }
        Err(e) => {
            eprintln!("error processing input: {}", e);
        }
    }
    Ok(())
}

// DAY 3 END

fn read_lines(input_path: &str) -> Result<Vec<String>, std::io::Error> {
    let input_file = fs::File::open(input_path)?;
    let lines = BufReader::new(input_file).lines();

    lines.collect()
}



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // let input_path =    "inputs/day3/test_input.txt";
    // let aoc_day = "day3".to_string();
    if args.len() < 3 {
        eprintln!("2 cmd argument required:\n - day of Advent of Code puzzle, in day1, day2,etc format\n - path to the input text file");
        process::exit(1);
    }
    let aoc_day = &args[1];
    let input_path = &args[2];

    let lines = read_lines(input_path)?;

    match aoc_day.as_str() {
        "day1" => day1_result(lines),
        "day2" => day2_result(lines),
        "day3" => day3_result(lines),
        _d => {
            eprintln!("Not implemented Advent Of Code Day selected: {}, currently only [day1,day2] are supported ", aoc_day);
            process::exit(1)
        }
    }
    // day1_result(lines)
    //   day2_result(lines)
}
