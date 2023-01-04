use core::fmt;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    env,
    error::Error,
    fs::{self},
    io::{BufRead, BufReader},
    num::ParseIntError,
    process,
};
use adv_of_code_2022::{day1, day2, day3, day4};

use pest::{Parser, iterators::Pair};
extern crate pest;
#[macro_use]
extern crate pest_derive;


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

// DAY 7

#[derive(Parser)]
#[grammar = "day7.pest"]
pub struct FsCmdParser;

#[derive(Debug)]
pub enum LsCmdOutput {
    DirOutput(String),
    FileOutput(String, usize),
}
#[derive(Debug)]
pub enum FsCmd {
    CdParent,
    CdRoot,
    Cd(String),
    Ls(Vec<LsCmdOutput>),
}

fn parse_cmds(e: Pair<Rule>) -> Result<FsCmd, &str> {
    match e.as_rule() {
        Rule::lsCmd => {            
            let output_lines: Result<Vec<LsCmdOutput>, &str> = e
            .into_inner()
            .map(|l| {
                match l.as_rule() {
                    Rule::dirOutput => 
                    Ok(LsCmdOutput::DirOutput(l.as_str().to_string())),
                    Rule::fileOutput => { 
                        let mut inner = l.into_inner();
                        let file_size =    inner.next().ok_or("lsCmd fileOutput is missing file size")?.as_str();
                        let file_name =    inner.next().ok_or("lsCmd fileOutput is missing file name")?.as_str();
                        let file_size = file_size.parse::<usize>().map_err(|_|"failed to parse file_size")?;
                        Ok(LsCmdOutput::FileOutput(file_name.to_string(), file_size))
                },
                _ => Err("invalid syntax for lsCmd output - expected dirOutput | fileOutput"),
            }
        })
            .collect();
            Ok(FsCmd::Ls(output_lines?))
        }
        Rule::cdCmd => { 
            let p =    e.into_inner().next().ok_or("Failed to find CD path")?;
            match p.as_rule() {
                Rule::cdRoot => Ok(FsCmd::CdRoot),
                Rule::cdParent => Ok(FsCmd::CdParent),
                Rule::cdPath => {
                    Ok(FsCmd::Cd(p.as_str().to_string()))
                },
                _ => Err("Invalid syntax, CdCmd can only contain cdRoot, cdParent or cdPath")
            }
           
        }
        _ => Err("Invalid top level cmd, only lsCmd or cdCmd are currently supported")
    }
}

struct ElfFs {
    cwd: Vec<String>,
    file_sizes: HashMap<Vec<String>, usize>,
}

impl ElfFs {
    fn empty() -> ElfFs {
        ElfFs { 
            cwd: vec!["/".to_string()],
            file_sizes: HashMap::new(),
          }
    }

    fn fold_cmd<'a>(acc: &'a mut ElfFs, cmd: &FsCmd) -> &'a mut ElfFs {
        match cmd {
            FsCmd::CdParent => acc.cd_parent(),
            FsCmd::CdRoot => acc.cd_root(),
            FsCmd::Cd(path) => acc.cd(path.clone()),
            FsCmd::Ls(contents) => acc.populate_files(contents),
        }
    }

    fn dir_sizes(&self) -> HashMap<String, usize> {
        let mut dirs = HashMap::new();

        for (path, size) in self.file_sizes.iter() {
            let path_parts = &path[0..path.len()-1];
            let absolute_paths = path_parts.iter().fold(vec![], |acc, p| {
                if acc.is_empty() {
                    vec![p.clone()]
                } else {
                    let parent =    acc.last().unwrap();
                    let path = format!("{}{}/", parent, p);
                    acc.into_iter().chain(vec![path].into_iter()).collect()
                }
            });
            for dir in absolute_paths {
                (*dirs.entry(dir.clone()).or_insert(0)) += size;
            }
        }
        dirs
    }

    fn populate_files(&mut self, cs: &Vec<LsCmdOutput>) -> &mut ElfFs {
        for o in cs {
            match o {
                LsCmdOutput::DirOutput(_) => (),
                LsCmdOutput::FileOutput(n, s) => {
                    let mut path = self.cwd.clone();
                    path.push(n.to_string());
                    self.file_sizes.insert(path, *s);
                } 
            }
        }
        self
    }
    
    fn cd_root(&mut self) -> &mut ElfFs {
        self.cwd = vec!["/".to_string()];
        self
    }

    fn cd_parent(&mut self) -> &mut ElfFs {
        self.cwd.pop();
        self
    }

    fn cd(&mut self, path: String) -> &mut ElfFs {
        self.cwd.push(path);
        self
    }
}

fn day7_result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let inputs =     lines.join("\n");
    let parsed = FsCmdParser::parse(Rule::fsCmd, &inputs[..])?;

    let cmds: Result<Vec<FsCmd>, &str> = parsed.map(parse_cmds).collect();
    let cmds = cmds?;
    let mut init = ElfFs::empty();
    let fs =    cmds.iter().fold(&mut init, ElfFs::fold_cmd);
    let dir_sizes = fs.dir_sizes();

    let result: usize = dir_sizes.iter()
        .filter(|(_, &size)| size <= 100000)
        .map(|(_, &size)| size)
        .sum();
    
        println!("Result: {}", result);
    let total_fs_size: usize = 70000000;
    let required_free_space: usize = 30000000;
    let root_dir_size = dir_sizes.get("/").ok_or("Cannot find / (root) in dir_sizes")?;
    let free_space = total_fs_size - root_dir_size;
    let space_needed_to_free = required_free_space - free_space;
    println!("Space needed to free: {}", space_needed_to_free);
    let mut big_enough_dirs:Vec<usize> =    dir_sizes.iter()
        .filter(|(_, &size)| size >= space_needed_to_free)
        .map(|(_, &s)|  s)
        .collect();
    big_enough_dirs.sort();
    let res_s = big_enough_dirs.first().ok_or("empty big_enough_dirs vec, no dirs are big enough!")?;
    println!("Part2 result:  {}", res_s);
        



    Ok(())
}

// DAY 7 END

fn read_lines(input_path: &str) -> Result<Vec<String>, std::io::Error> {
    let input_file = fs::File::open(input_path)?;
    let lines = BufReader::new(input_file).lines();

    lines.collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("2 cmd argument required:\n - day of Advent of Code puzzle, in day1, day2,etc format\n - path to the input text file");
        process::exit(1);
    }
    let aoc_day = &args[1];
    let input_path = &args[2];

    let lines = read_lines(input_path)?;

    match aoc_day.as_str() {
        "day1" => day1::result(lines),
        "day2" => day2::result(lines),
        "day3" => day3::result(lines),
        "day4" => day4::result(lines),
        "day5" => day5_result(lines),
        "day6" => day6_result(lines),
        "day7" => day7_result(lines),
        _d => {
            eprintln!("Not implemented Advent Of Code Day selected: {}, currently only [day1,day2] are supported ", aoc_day);
            process::exit(1)
        }
    }
    // day1_result(lines)
    // day2_result(lines)
}
