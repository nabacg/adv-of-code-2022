use std::error::Error;

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

pub fn result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut elves = parse_elves(lines)?;

    println!("result is: {}", elves.top_three_total());
    Ok(())
}

// DAY 1 END
