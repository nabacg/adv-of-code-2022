use std::{error::Error, num::ParseIntError, collections::HashMap};
use pest::{Parser, iterators::Pair};

#[derive(Debug, Clone)]
struct MonkeyTest {
    param: i32,
    truthy_target: usize,
    falsy_target: usize,
}
impl MonkeyTest {
    fn apply(&self, new_worry_level: i32) -> usize {
        if new_worry_level % self.param == 0{
            self.truthy_target
        } else {
            self.falsy_target
        }
    }
}

#[derive(Debug, Clone)]
enum MonkeyOpArg {
    Old,
    IntArg(i32),
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

    fn apply(&self, worry_level: i32) -> i32 {
        match self.operator.as_str() {
            "+" => self.arg0(worry_level) + worry_level,
            "*" => self.arg0(worry_level) * worry_level,
            _ => panic!("invalid Opertor found, only (+ | * ) are suuported, got {}", self.operator),
        }
    }

    fn arg0(&self, arg1: i32) -> i32 {
        match self.arg {
            MonkeyOpArg::Old => arg1,
            MonkeyOpArg::IntArg(arg0) => arg0,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    operation: MonkeyOp,
    test: MonkeyTest,
    inspected_items: usize,
}

impl Monkey {
    fn new(id: usize, items: Vec<i32>, op: MonkeyOp, test: MonkeyTest) -> Monkey {
        Monkey{ id: id,
             items: items, 
             operation: op, 
             test: test, 
             inspected_items:0 }
    }

    fn next_round(&self, inspected: usize) -> Monkey {
        Monkey{ id: self.id,
            items: vec![], 
            operation: self.operation.clone(), 
            test: self.test.clone(), 
            inspected_items:self.inspected_items + inspected }
    }

    fn round(&self) -> (Monkey, Vec<(usize, i32)>) {
        let thrown_items: Vec<(usize, i32)> =        self.items.iter().map(|&i| {
            let new_worry_level = self.operation.apply(i);
            let new_worry_level = new_worry_level / 3; // This operation rounds towards zero,  so I think we're fine?
            let target_monkey = self.test.apply(new_worry_level);
            (target_monkey, new_worry_level)
        }).collect();

        (self.next_round(thrown_items.len()),   thrown_items)
    }

    fn catch_item(&mut self, item: i32) {
        self.items.push(item);
    }
}

struct MonkeyGame {
    monkeys: Vec<Monkey>,

}

impl MonkeyGame {
    fn new(monkeys: Vec<Monkey>) -> MonkeyGame {
        MonkeyGame { monkeys, }
    }
    fn round(&mut self) {
        for m in &self.monkeys {
            let (new_m, thrown_items) = m.round();
            println!("new_m: {:?}, thrown_items: {:?}", new_m, thrown_items);
            // for (target_monkey_id, item) in thrown_items {
            //     let target_monkey = self.monkeys.get(&target_monkey_id).unwrap();
            //     let mut new_target_monkey = target_monkey.clone(); 
            //     (&new_target_monkey).catch_item(item);
            // }
        }
    }
}

#[derive(Parser)]
#[grammar = "day11.pest"]
struct MonkeySpecParser;

pub(crate) fn result(input: String) -> Result<(), Box<dyn Error>> {
    let monkeys = parse_input(&input)?;
    let mut mg = MonkeyGame::new(monkeys);
    mg.round();
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
                let starting_items: Result<Vec<i32>, String> =   starting_items_spec.into_inner()
                    .map(|i| i.as_str().parse::<i32>().map_err(|e| format!("{}", e))).collect();
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
                   Rule::operationIntArg =>  Ok(MonkeyOpArg::IntArg(param_p.as_str().parse::<i32>().map_err(|e| format!("{}", e))?)),
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
                let digits: Result<Vec<i32>, String> =        test_spec.into_inner()
                    .map(|d| d.as_str().parse::<i32>().map_err(|e| format!("{}", e)))
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