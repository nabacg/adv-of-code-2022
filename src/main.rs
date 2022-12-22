use std::{fs, io::{BufReader, BufRead}, env, process};

struct ElfExpedition {
    max_calories: i32,
    current_sum: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("1 cmd argument required: provide path to the input file");
        process::exit(1);
    }
    let input_path = &args[1];

    let input_file = fs::File::open(input_path).expect("failed to load input.txt");
    let lines = BufReader::new(input_file).lines();

    let res = lines.fold(ElfExpedition{max_calories: 0,  current_sum: 0}, |acc, l| {
        match l {
            Ok(l) if l.len() == 0 => if acc.current_sum > acc.max_calories {
                ElfExpedition{max_calories: acc.current_sum, current_sum: 0}
            } else {
                ElfExpedition{max_calories: acc.max_calories, current_sum: 0}
            },
            Ok(l) => {
                let cals = l.parse::<i32>().unwrap();
                ElfExpedition{max_calories: acc.max_calories, current_sum: acc.current_sum + cals}
            },
            _ee => panic!("how did this happen??")
        }

    });

    println!("result is: {}", res.max_calories);
    // for l in lines {
    //     println!("line: {}", l.unwrap());
    // }
}
