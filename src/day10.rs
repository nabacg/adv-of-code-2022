use std::{error::Error, num::ParseIntError, collections::HashSet};

enum Inst {
    AddX(i32),
    NoOp,
}

impl Inst {
    fn new(l: &String) -> Result<Inst, String> {
        if l.as_str() == "noop" {
            Ok(Inst::NoOp)
        } else {
            let parts: Vec<&str> = l.split(" ").collect();
            if parts.len() != 2 || parts[0] != "addx" {
                Err(format!("invalid input line - expected 'addx N', got: {}", l))
            } else {
                let n = parts[1].parse::<i32>()
                    .map_err(|e| format!("Invalid Input - can't parse addx arg: {}", e))?;
                Ok(Inst::AddX(n))
            }
        }
    }
}

struct CPU {
    X: i32,
    CurrentCycle: u32,
    SignalRecording: Vec<(u32, i32)>,
    CRT_output: Vec<char>,
}

impl CPU {
    fn run(&mut self, i: &Inst) {
        self.CurrentCycle += 1;
        self.check_signal();
        match i {
            Inst::AddX(n) => {                
                self.CurrentCycle += 1;
                self.check_signal();
                self.X += n;
            },
            Inst::NoOp => {},
        }
    }

    fn draw_sprite(&self) -> char {
        let crt_pixel = (self.CurrentCycle -1) as i32; // The left-most pixel in each row is in position 0, and the right-most pixel in each row is in position 39.
        let crt_pixel_norm = crt_pixel % 40; // convert absolute CRT pixel position (0, 239) to current row (0, 39)
        if crt_pixel_norm.abs_diff(self.X) <= 1 {
            // println!("# - Cycle: {}, X: {}", self.CurrentCycle, self.X);
            '#'
        } else {
            // println!(". - Cycle: {}, X: {}", self.CurrentCycle, self.X);
            '.'
        }
    }



    fn check_signal(&mut self) {
        // println!("check_signal: {}", self.CurrentCycle);
        if self.CurrentCycle == 20 || (self.CurrentCycle > 20 && (self.CurrentCycle - 20) % 40 == 0) {
            // println!("Interesting Signal, Cycle: {}, X:{}", self.CurrentCycle, self.X);
            self.SignalRecording.push((self.CurrentCycle, self.X));
        }
        
        self.CRT_output.push(self.draw_sprite());

        if self.CurrentCycle % 40 == 0 {
            self.CRT_output.push('\n')
        }
    }

    fn signal_strengths(&self) -> Vec<i32> {
        self.SignalRecording.iter().map(|(cc, x)| (*cc as i32) * x).collect()
    }

    fn new() -> CPU {
        CPU { X: 1, CurrentCycle: 0, SignalRecording: vec![], CRT_output: vec![] }
    }
}

pub(crate) fn result(ls: Vec<String>) -> Result<(), Box<dyn Error>> {
    let program: Result<Vec<Inst>, String> = ls.iter().map(Inst::new).collect();
    let program = program?;
    let mut cpu = CPU::new();

    program.iter().for_each(|i| {
        cpu.run(i);
    });

    let signal_strengths:i32 = cpu.signal_strengths().iter().sum();
    println!("Par1 Result: {}", signal_strengths);

    let crt_out:String = cpu.CRT_output.into_iter().collect();
    println!("{}", crt_out);
    Ok(())
}