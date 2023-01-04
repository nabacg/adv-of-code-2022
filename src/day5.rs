use core::fmt;
use std::{error::Error, num::ParseIntError, collections::HashMap, process};


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

pub fn result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {

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