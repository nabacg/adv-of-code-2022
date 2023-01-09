use core::fmt;
use std::{error::Error, num::ParseIntError, collections::HashMap};
use pest::{Parser, iterators::Pair};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct MonkeyTest {
    param: u64,
    truthy_target: usize,
    falsy_target: usize,
}
impl MonkeyTest {
    fn apply(&self, new_worry_level: u64) -> usize {
        if new_worry_level % self.param == 0 {
            self.truthy_target
        } else {
            self.falsy_target
        }
    }
}

#[derive(Debug, Clone)]
enum MonkeyOpArg {
    Old,
    IntArg(u64),
}

#[derive(Debug, Clone)]
struct MonkeyOp {
    arg: MonkeyOpArg,
    operator: String,
}
impl MonkeyOp {
    fn new(op: String, arg: MonkeyOpArg) -> MonkeyOp {
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
struct Monkey {
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
    fn new(id: usize, items: Vec<u64>, op: MonkeyOp, test: MonkeyTest) -> Monkey {
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

#[derive(Parser)]
#[grammar = "day11.pest"]
struct MonkeySpecParser;

pub(crate) fn result(input: String) -> Result<(), Box<dyn Error>> {
    let monkeys = parse_input(&input)?;
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

fn parse_input(input: &str) -> Result<Vec<Monkey>, String> {
    let parsed = MonkeySpecParser::parse(Rule::monkeySpecs, input).map_err(|e| format!("{}",e))?;

    let monkeys: Result<Vec<Monkey>, String> = parsed.map(parse_monkey).collect();

    monkeys.map_err(|e| format!("{}",e))
}

fn parse_monkey(p: Pair<Rule>) -> Result<Monkey, String> {
    let mut pairs =    p.into_inner();
    let monkey_spec = pairs.next().ok_or("Invalid input expected monkeyId")?;
    let m_id = match monkey_spec.as_rule() {
        Rule::monkeyId => monkey_spec.as_str().parse::<usize>().map_err(|e| format!("{}", e)),
        _ => Err("Invalid input, expected ASCII_DIGIT+ representing monkeyId".into())
    };
    let starting_items_spec = pairs.next().ok_or("Invalid input expected startingItemsSpec")?;
    let starting_items = match starting_items_spec.as_rule() {
        Rule::startingItemsSpec => {
                let starting_items: Result<Vec<u64>, String> =   starting_items_spec.into_inner()
                    .map(|i| i.as_str().parse::<u64>().map_err(|e| format!("{}", e))).collect();
                let starting_items  = starting_items?;
                Ok(starting_items)                

        },
        _ => Err(format!("Invalid input, expected startingItemsSpec, got: {}", starting_items_spec.as_str()))
    };

    let operation_spec = pairs.next().ok_or("Invalid input - expected operationSpec")?;
    let operation_coef:Result<MonkeyOp, String> = match operation_spec.as_rule() {
        Rule::operationSpec => {
            let mut operation_params = operation_spec.into_inner();
            let operator =   operation_params.next()
                .ok_or("Invalid input, operation_spec requires operation Operator (+ | *) ")?
                .as_str()
                .to_string();
                
            let coef:Result<MonkeyOpArg, String> =   {
                let param_p = operation_params.next().ok_or("Invalid input, operation_spec needs operationCoef ")?;
                match param_p.as_rule() {
                   Rule::operationIntArg =>  Ok(MonkeyOpArg::IntArg(param_p.as_str().parse::<u64>().map_err(|e| format!("{}", e))?)),
                   Rule::operationOldSelfArg => Ok(MonkeyOpArg::Old),
                   _ => Err("Invalid input, operationSpec operationArg".into()),
                }
            };  

                Ok(MonkeyOp::new(operator, coef?))
            },
        _ => Err("Invalid input, expected ASCII_DIGIT+ representing monkeyId".into())
    };


    let test_spec = pairs.next().ok_or("Invalid input - expected testSpec")?;
    let monkey_test:Result<MonkeyTest, String> = match test_spec.as_rule() {
             Rule::testSpec => {
                let digits: Result<Vec<u64>, String> =        test_spec.into_inner()
                    .map(|d| d.as_str().parse::<u64>().map_err(|e| format!("{}", e)))
                    .collect();
                let digits = digits?;
                Ok(MonkeyTest{
                    param: digits[0],
                    truthy_target: digits[1] as usize,
                    falsy_target: digits[2] as usize,
                })
            }
            _ => Err("Invalid input, expected ASCII_DIGIT+ representing monkeyId".into())
    };


    // println!("MonkeyId: {}, starting_items: {:?}, monkey_test: {:?}", m_id?, starting_items?, monkey_test?);
    Ok(Monkey::new(m_id?, starting_items?,  operation_coef?,  monkey_test?))
}



#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn parse_monkeys_test() {
        let test_spec  = 
"Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3

      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0

      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3

      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1";

        let res = parse_input(test_spec);
        println!("res={:?}", res);
        assert_eq!(res.is_ok(), true);
        let monkeys = res.unwrap();
        assert_eq!(monkeys.len(), 4);


    }


    #[test]
    fn parse_monkey_test() {
        let input  = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old + 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

        let res = parse_monkey(MonkeySpecParser::parse(Rule::monkeySpec, input).unwrap().next().unwrap());
        println!("res={:?}", res);
        assert_eq!(res.is_ok(), true);


    }
}