use core::fmt;
use std::{
    arch::x86_64::_mm256_and_pd,
    collections::{HashMap, HashSet, VecDeque},
    env,
    error::Error,
    fs::{self},
    io::{BufRead, BufReader},
    num::ParseIntError,
    process,
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
#[derive(Debug)]
pub struct Rucksack {
    compartment_a: Vec<char>,
    compartment_b: Vec<char>,
}

impl Rucksack {
    pub fn from_input(l: &String) -> Result<Rucksack, &str> {
        if l.chars().count() % 2 != 0 {
            return Err("Invalid input, only even character counts are supported");
        }
        let (a, b) = l.split_at(l.chars().count() / 2);
        Ok(Rucksack {
            compartment_a: a.chars().collect(),
            compartment_b: b.chars().collect(),
        })
    }

    pub fn compartment_overlaps(&self) -> Vec<&char> {
        // let mut item_count: HashMap<&char, i32> = HashMap::new();
        // self.compartment_a.iter().for_each(|i| *item_count.entry(i).or_insert(0) += 1);
        // self.compartment_b.iter().for_each(|i| *item_count.entry(i).or_insert(0) += 1);
        // item_count.iter().filter(|(i, &c)| c > 1).map(|(i, _)| *i).collect()

        let comp_a_set = HashSet::<_>::from_iter(self.compartment_a.iter());
        let dedup =
            HashSet::<_>::from_iter(self.compartment_b.iter().filter(|b| comp_a_set.contains(b)));
        dedup.iter().map(|c| *c).collect()
    }

    pub fn overlaps<'a>(&'a self, contents_b: &'a Vec<&char>) -> Vec<&'a char> {
        let self_invent = HashSet::<_>::from_iter(self.contents());
        let other_invent = HashSet::<_>::from_iter(contents_b.iter().map(|i| *i));
        self_invent
            .intersection(&other_invent)
            .map(|c| *c)
            .collect()
    }

    pub fn contents(&self) -> Vec<&char> {
        self.compartment_a
            .iter()
            .chain(self.compartment_b.iter())
            .collect()
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

fn intersection<'a>(contents_a: Vec<&'a char>, contents_b: Vec<&'a char>) -> Vec<&'a char> {
    let self_invent = HashSet::<_>::from_iter(contents_a.iter().map(|i| *i));
    let other_invent = HashSet::<_>::from_iter(contents_b.iter().map(|i| *i));
    self_invent
        .intersection(&other_invent)
        .map(|c| *c)
        .collect()
}

fn day3_result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let rucksacks: Result<Vec<Rucksack>, &str> = lines.iter().map(Rucksack::from_input).collect();
    match rucksacks {
        Ok(rucksacks) => {
            // let misplaced_items: u32 = rucksacks.iter()
            //     .flat_map(|r| r.compartment_overlaps())
            //     .map(Rucksack::item_priority)
            //     .sum();
            // let groups =        rucksacks.iter().next_chunk::<3>().expect("invalid input - number of lines need to be divisable by 3");
            let group_badges: u32 = rucksacks.chunks(3).map(|g| {
                    let group_overlaps = g.iter().map(|r|r.contents()).reduce(intersection).expect("Invalid input - empty elf group provided, expected 3 elves in each group.");

                    if group_overlaps.len() != 1 {
                        eprintln!("error processing input, only single badge should overlap in each group, got: {}", group_overlaps.len());
                        process::exit(1)
                    }
                    group_overlaps[0]
                }).map(|i| Rucksack::item_priority(i)).sum();

            println!("result is {}", group_badges);
        }
        Err(e) => {
            eprintln!("error processing input: {}", e);
        }
    }
    Ok(())
}

// DAY 3 END

// DAY 4
#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn from_input(i: &str) -> Result<Section, Box<dyn Error>> {
        let edges: Result<Vec<u32>, ParseIntError> =
            i.split("-").map(|e| e.parse::<u32>()).collect();
        let edges = edges?;
        if edges.len() != 2 {
            Err(format!(
                "Invalid input, expected 2 part assigment separated by ',', got: {}",
                i
            )
            .into())
        } else {
            Ok(Section {
                start: edges[0],
                end: edges[1],
            })
        }
    }

    fn contains(&self, another: &Section) -> bool {
        self.start <= another.start && self.end >= another.end
    }

    fn left_overlap(&self, another: &Section) -> bool {
        another.end >= self.start && another.start <= self.end
    }

    fn overlap(&self, another: &Section) -> bool {
        // left overlap
        // ...456...  4-6
        // .234.....  2-4
        self.left_overlap(another) ||
        //right overlap
        // .234.....  2-4
        // ...456...  4-6
         another.left_overlap(self)
    }
}

#[derive(Debug)]
struct CleaningAssignment {
    left: Section,
    right: Section,
}

impl CleaningAssignment {
    fn from_input(l: &String) -> Result<CleaningAssignment, Box<dyn Error>> {
        let parts: Vec<&str> = l.split(",").collect();
        if parts.len() != 2 {
            Err(format!(
                "Invalid input, expected 2 part assigment separated by ',', got: {}",
                l
            )
            .into())
        } else {
            let l = Section::from_input(parts[0])?;
            let r = Section::from_input(parts[1])?;
            Ok(CleaningAssignment { left: l, right: r })
        }
    }

    fn pair_fully_contains(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn pair_overlap(&self) -> bool {
        self.left.overlap(&self.right)
    }
}

fn day4_result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let overlapping: Result<Vec<CleaningAssignment>, Box<dyn Error>> =
        lines.iter().map(CleaningAssignment::from_input).collect();
    let overlapping_count = overlapping?
        .iter()
        .filter(|a| a.pair_overlap())
        .map(|i| {
            //  println!("overlap: {:?}", i);
            i
        })
        .count();
    println!("Result is: {}", overlapping_count);

    Ok(())
}

// DAY 4 END

// DAY 5 
#[derive(Debug)]
struct Stack {
    id: usize,
    crates: Vec<char>,
}

impl Stack {
    fn new(id: usize)-> Stack {
        return Stack { id: id, crates: Vec::new() }
    }
    fn push(&mut self, c: char) {
        self.crates.push(c)
    }

    fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    fn pop_n(&mut self, n: i32) -> Option<Vec<char>> {
        let mut res =   vec![];
        for _i in 0..n {
            let r = self.pop()?;
            res.push(r);
        }
        Some(res)
    }

    fn push_n(&mut self, cs: Vec<char>) {
        cs.iter().rev().for_each(|&c| self.push(c));
    }
}

#[derive(Debug)]
struct MoveCmd {
    count: i32,
    source: usize,
    destination: usize,
}

impl MoveCmd {
    fn from_input( ls: &String) -> Result<MoveCmd, Box<dyn Error>> {
        let collect = ls.split_whitespace().collect::<Vec<&str>>();
        if let [_, count, _, source, _, destination] = collect.as_slice() {
            let cmd = MoveCmd {
                count: count.parse()?,
                source: source.parse()?,
                destination: destination.parse()?,
            };
            Ok(cmd)
        } else {
            Err("Invalid MoveCmd input line".into())
        }


    }
}

impl fmt::Display for MoveCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.count, self.source, self.destination)
    }
}

#[derive(Debug)]
struct SupplyStacks {
    stacks: HashMap<usize, Stack>,
}

impl fmt::Display for SupplyStacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_crate =    self.stacks.values().map(|v| v.crates.len()).max();
        let mut stack_idx: Vec<usize> = self.stacks.keys().map(|&k| k).collect();
        stack_idx.sort();

        if let Some(mc) = max_crate {
            for l in (0..mc).rev() {
                let line: Option<Vec<String>> = stack_idx.iter()

                    .map(|k| { 
                        let v = self.stacks.get(k)?;
                        v.crates.get(l)
                    })
                     .map(|c| c.map(|c| format!("[{}]", c)).or(Some("   ".to_string())))
                    .collect();

                if let Some(cs) = line {
                    write!(f, "{}\n", cs.join(" "))?
                }
            }
        }
        write!(f, " {} ", stack_idx.iter().map(|k| k.to_string()).collect::<Vec<String>>().join("   "))
    }
}

impl SupplyStacks {
    fn from_input(mut ls: Vec<&String>) -> Result<SupplyStacks, Box<dyn Error>> {
        ls.reverse();
        let stack_nums:Result<Vec<usize>, ParseIntError> = ls[0].split_whitespace().map(|i| i.parse::<usize>()).collect(); 
        // let stacks:Vec<Stack> = stack_nums?.iter().map(|i| Stack::new(*i)).collect();

        let mut stacks_map: HashMap<usize, Vec<char>> = stack_nums?.iter().map(|&i|(i,vec![])).collect();

        ls[1..].iter().for_each(|l| SupplyStacks::push_line(&mut stacks_map, l));

        let supply_stacks = SupplyStacks { stacks: stacks_map.iter().map(|(&k,v)| {
            let mut s = Stack::new(k );
            v.iter().for_each(|&c| s.push(c));
            (k, s)
        }).collect() 
    };
        return Ok(supply_stacks)
    }

  
    fn push_line(stacks_map: &mut HashMap<usize, Vec<char>>, l: &str) {
        let mut stack_ids:Vec<&usize> =        stacks_map.keys().collect(); // HashMap.keys() returns keys in ARBITRARY order
        stack_ids.sort();
        let input_column:Vec<usize> = (0..stack_ids.len()).fold(vec![1], |mut acc, _i| {
            let last = acc.last().expect("can't get last on a non empty vector");
            acc.push(last + 4);
            acc
        });
        let stack_2_column:HashMap<usize, usize> = input_column.iter().zip(stack_ids).map(|(&s_id, &idx)| (s_id, idx)).collect();

        l.chars()
        .enumerate()
        .filter(|(i, _c)| stack_2_column.contains_key(i))
        .for_each(|(i, c)| {
            // filter out whitespace which represents lack of crate
            if c != ' ' {
                let &k = stack_2_column.get(&i).expect("failed to get stack idx key ");
                stacks_map.entry(k).or_insert(vec![]).push(c)
            }
        });

    }

    fn apply(&mut self, cmd: &MoveCmd) -> Option<()>{

        let source = self.stacks.get_mut(&cmd.source)?;
        let cs = source.pop_n(cmd.count)?;

        let target = self.stacks.get_mut(&cmd.destination)?;
        target.push_n(cs);
        // for _i in 0..cmd.count {
        //     let source = self.stacks.get_mut(&cmd.source)?;
        //     // let c = source.pop()?;
        //     // let target = self.stacks.get_mut(&cmd.destination)?;
        //     let cs = source.pop_n(cmd.count)
        //     target.push(c);
        // }
        Some(())
    }

    fn top_of_stacks(&self) -> String {
        let mut top_crates:Vec<(usize,&char)> =        self.stacks
        .values()
        .map(|vs| (vs.id, vs.crates.last().ok_or("").unwrap()))
        .collect();
        top_crates.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));

        top_crates.iter().map(|(_, s)| s.to_string()).collect::<Vec<String>>().join("")
    }
}

fn day5_result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {

    let stacks_inputs:Vec<&String> = lines.iter().take_while(|&l| l != "").collect();
    let cmd_inputs:Vec<&String>  = lines.iter().skip_while(|&l| l != "").skip(1).collect();
    let mut stacks = SupplyStacks::from_input(stacks_inputs)?;
    println!("SupplyStacks:\n{}", stacks);
    let cmds:Result<Vec<MoveCmd>, Box<dyn Error>> = cmd_inputs.iter().map(|&l| MoveCmd::from_input(l)).collect();
    let cmds = cmds?;
    // println!("cmds: {:?}", cmds);
    cmds.iter().for_each(|cmd| {
        let r = stacks.apply(cmd);
        println!("\n{}\n{}", cmd, stacks);
        if r.is_none() {
            eprintln!("Failed to apply cmd:\n{}\n, stacks:\n{}", cmd, stacks);
            process::exit(1)
        }
    });
    println!("Result: {}", stacks.top_of_stacks());
    Ok(())

}
// DAY 5 END

//DAY 6
struct MarkerDetector {
    ring_buffer: VecDeque<char>,
    chars_processed: usize,
    marker_length: usize,
}

impl MarkerDetector {
    fn new(l: usize) -> MarkerDetector {
        return MarkerDetector {
                marker_length: l,
                ring_buffer: VecDeque::with_capacity(l), 
                chars_processed: 0
             }
    }

    fn process(&mut self, c: char) -> bool {
        if self.ring_buffer.len() >= self.marker_length {
            self.ring_buffer.pop_front();
        }
        self.ring_buffer.push_back(c);

        self.chars_processed += 1;
        HashSet::<_>::from_iter(self.ring_buffer.iter()).len() == self.marker_length
    }
}

fn day6_result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let marker_len = 14;
    for l in    lines {
        let mut sop = MarkerDetector::new(marker_len);
        l.chars().take_while(|&c| !sop.process(c)).count();
        println!("Result: {}", sop.chars_processed);
    }
    Ok(())
}

// DAY 6 END

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
        "day4" => day4_result(lines),
        "day5" => day5_result(lines),
        "day6" => day6_result(lines),
        _d => {
            eprintln!("Not implemented Advent Of Code Day selected: {}, currently only [day1,day2] are supported ", aoc_day);
            process::exit(1)
        }
    }
    // day1_result(lines)
    // day2_result(lines)
}
