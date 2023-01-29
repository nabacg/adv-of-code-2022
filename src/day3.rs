use std::{collections::HashSet, error::Error, process};

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

pub fn result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
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
