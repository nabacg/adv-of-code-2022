use core::fmt;
use std::{error::Error, num::ParseIntError, collections::HashMap};

use itertools::Itertools;

use crate::day11::parser;

#[derive(Debug, Clone)]
pub(crate) struct MonkeyTest {
    param: u64,
    truthy_target: usize,
    falsy_target: usize,
}
impl MonkeyTest {
    pub(crate) fn new(p: u64, truthy_t: usize, falsy_t: usize) -> MonkeyTest {
        MonkeyTest { param: p, truthy_target: truthy_t, falsy_target: falsy_t, }
    }

    fn apply(&self, new_worry_level: u64) -> usize {
        if new_worry_level % self.param == 0 {
            self.truthy_target
        } else {
            self.falsy_target
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum MonkeyOpArg {
    Old,
    IntArg(u64),
}

#[derive(Debug, Clone)]
pub(crate) struct MonkeyOp {
    arg: MonkeyOpArg,
    operator: String,
}
impl MonkeyOp {
    pub(crate) fn new(op: String, arg: MonkeyOpArg) -> MonkeyOp {
        MonkeyOp { arg, operator: op }
    }

    fn apply(&self, worry_level: u64) -> u64 {
        match self.operator.as_str() {
            "+" => self.arg0(worry_level) + worry_level,
            "*" =>  self.arg0(worry_level) * worry_level,
            _ => panic!("invalid Opertor found, only (+ | * ) are suuported, got {}", self.operator),
        }
    }

    fn arg0(&self, arg1: u64) -> u64 {
        match self.arg {
            MonkeyOpArg::Old => arg1.clone(),
            MonkeyOpArg::IntArg(arg0) => arg0,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: MonkeyOp,
    test: MonkeyTest,
    inspected_items: usize,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Items: {:?}, Inspected: {}", self.items, self.inspected_items)
    }
}

impl Monkey {
    pub fn new(id: usize, items: Vec<u64>, op: MonkeyOp, test: MonkeyTest) -> Monkey {
        Monkey{ id: id,
             items: items, 
             operation: op, 
             test: test, 
             inspected_items:0 }
    }

    fn round(&self, div: &u64) -> (usize, Vec<(usize, u64)>) {
        let thrown_items: Vec<(usize, u64)> =        self.items.iter().map(|&i| {
            let new_worry_level = self.operation.apply(i);
            //let new_worry_level = new_worry_level / 3; // This operation rounds towards zero,  so I think we're fine?

            // use Chinese Reminder Theorem https://brilliant.org/wiki/chinese-remainder-theorem/ 
            // since all divisors are prime, we can find their GCD by multiplying them and then use this GCD to modulo the worry level
            //  without affecting divisability test
            // thanks Reddit https://www.reddit.com/r/adventofcode/comments/zifqmh/comment/j26b81u/?utm_source=share&utm_medium=web2x&context=3
            let new_worry_level = new_worry_level % div;
            let target_monkey = self.test.apply(new_worry_level);
            (target_monkey, new_worry_level)
        }).collect();

        (thrown_items.len(),   thrown_items)
    }

    fn catch_item(&mut self, item: u64) {
        self.items.push(item);
    }
}

struct MonkeyGame {
    monkeys: Vec<Monkey>,
    gcd: u64,

}

impl MonkeyGame {

    fn print(&self) {
        let ms = self.monkeys.iter().map(|m| format!("{}", m)).collect::<Vec<String>>();
        println!("Monkeys:\n{}\n", ms.join(""));
    }

    fn new(monkeys: Vec<Monkey>) -> MonkeyGame {
        let gcd = monkeys.iter().map(|m| m.test.param).product();
        MonkeyGame { monkeys, gcd}
    }

    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            // self.print();
            let (inspected_items, thrown_items) = self.monkeys[i].round(&self.gcd);
            //  println!("inspected_items: {}, thrown_items: {:?}", inspected_items, thrown_items);
            for (target_monkey_id, item) in thrown_items {
                let target_monkey =
                                 self.monkeys
                                    .get_mut(target_monkey_id)
                                    .expect("Item thrown to unknown Monkey.");
                target_monkey.catch_item(item);
            }
            let update_m = self.monkeys.get_mut(i).unwrap();
            update_m.inspected_items += inspected_items;
            update_m.items.clear();


           
        }
    }
}


pub(crate) fn result(input: String) -> Result<(), Box<dyn Error>> {
    let monkeys = parser::parse_input(&input)?;
    let mut mg = MonkeyGame::new(monkeys);
    for i in 0..10000 {
        println!("Round: {}", i);
        mg.round();
    }
    mg.print();
    let monkey_business:Vec<usize> = mg.monkeys.iter().map(|m| m.inspected_items).sorted().rev().take(2).collect();
    println!("Part 1 Result: {:?}", monkey_business.iter().product::<usize>());
    Ok(())
}
