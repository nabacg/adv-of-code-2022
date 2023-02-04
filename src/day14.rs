use core::fmt;
use itertools::Itertools;
use std::{collections::HashSet, error::Error, num::ParseIntError};

struct RockPath {
    path_lines: Vec<((i32, i32), (i32, i32))>,
    lowest_level: i32,
    left_most: i32,
    right_most: i32,
}

impl RockPath {
    fn is_blocked(&self, (p_x, p_y): (i32, i32)) -> bool {
        self.path_lines.iter().any(|((e_x, e_y), (s_x, s_y))| {
            let r = (s_x <= &p_x && &p_x <= e_x || e_x <= &p_x && &p_x <= s_x)
                && (e_y <= &p_y && &p_y <= s_y || s_y <= &p_y && &p_y <= e_y);
            //    println!("is_blocked: {}, because ({}, {}) is between ({},{}) and ({},{})", r, p_x, p_y, s_x, s_y, e_x, e_y);
            r
        })
    }

    fn new(p: Vec<(i32, i32)>) -> RockPath {
        let lowest_level = *p.iter().map(|(_, y)| y).max().unwrap();
        let left_most = *p.iter().map(|(x, _)| x).filter(|&x| x > &i32::MIN).min().unwrap();
        let right_most = *p.iter().map(|(x, _)| x).filter(|&x| x < &i32::MAX).max().unwrap();
        let path_lines = p
            .iter()
            .zip(p.iter().skip(1))
            .into_iter()
            .map(|(&s, &e)| (s, e))
            .collect::<Vec<((i32, i32), (i32, i32))>>();
        RockPath {
            path_lines,
            lowest_level,
            left_most,
            right_most,
        }
    }
}

type Sand = (i32, i32);

struct Cave {
    sand_units: HashSet<(i32, i32)>,
    rock_paths: Vec<RockPath>,
    sand_unit_total: usize,
    bottom_level: i32,
    left_most: i32,
    right_most: i32,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            (self.left_most..self.right_most + 1)
                .into_iter()
                .map(|c| if c == self.left_most || c == self.right_most {
                    c.to_string()
                } else {
                    " ".to_string()
                })
                .join("")
        )
        .expect("failed to write to fmt:Formatter");

        let left_bound =  self.left_most.min(self.sand_units.iter().map(|(x,_)| *x).min().or(Some(self.left_most)).unwrap());
        let right_bound =  self.right_most.min(self.sand_units.iter().map(|(x,_)| *x).max().or(Some(self.right_most)).unwrap());
        for y in 0..self.bottom_level + 3 {
            let line = (left_bound-5..right_bound+15)
                .into_iter()
                .map(|x| match x {
                    x if self.sand_units.contains(&(x, y)) => 'o',
                    x if self.rock_paths.iter().any(|p| p.is_blocked((x, y))) => '#',
                    _ => '.',
                })
                .join("");
            writeln!(f, "{} {}", y, line).expect("failed to write into fmt:Formatter");
        }
        Ok(())
    }
}

impl Cave {
    fn new_sand() -> Sand {
        (500, 0)
    }

    fn move_sand(&mut self) -> Sand {
        let mut s = Cave::new_sand();
        let mut n_s = self.next_sand_pos(s);
        while s != n_s && self.above_bottom_rock(n_s) {
            //  println!("sand: {:?}, next_sand: {:?}", s, n_s);
            //  self.sand_units.insert(s);
            //  println!("{}", self);
            //  self.sand_units.remove(&s);
            let tmp = n_s;
            n_s = self.next_sand_pos(n_s);
            s = tmp;
        }
        self.sand_units.insert(n_s);
        self.sand_unit_total += 1;
        n_s
    }

    fn next_sand_pos(&self, (x, y): Sand) -> Sand {
        if self.is_free((x, y + 1)) {
            // go down
            (x, y + 1)
        } else if self.is_free((x - 1, y + 1)) {
            // go left
            (x - 1, y + 1)
        } else if self.is_free((x + 1, y + 1)) {
            // go right
            (x + 1, y + 1)
        } else {
            (x, y)
        }
    }

    fn is_free(&self, y: (i32, i32)) -> bool {
        !self.sand_units.contains(&y) && self.rock_paths.iter().all(|p| !p.is_blocked(y))
    }

    fn above_bottom_rock(&self, (_, y): (i32, i32)) -> bool {
        y <= self.bottom_level
    }

    pub(crate) fn new(paths: Vec<Vec<(i32, i32)>>) -> Cave {
        let mut rock_paths: Vec<RockPath> = paths.into_iter().map(|p| RockPath::new(p)).collect();
        let bottom_level = rock_paths.iter().map(|rp| rp.lowest_level).max().unwrap();
        let left_most = rock_paths.iter().map(|rp| rp.left_most).min().unwrap();
        let right_most = rock_paths.iter().map(|rp| rp.right_most).max().unwrap();
        // bottom floor
        rock_paths.push(RockPath::new(vec![(i32::MIN,bottom_level+2), (i32::MAX, bottom_level+2)]));


        Cave {
            sand_units: HashSet::new(),
            rock_paths,
            sand_unit_total: 0,
            bottom_level,
            left_most,
            right_most,
        }
    }
}

fn parse_point(p: &str) -> Result<(i32, i32), String> {
    let ps = p
        .split(",")
        .map(|i| i.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>();

    ps.map_err(|s| format!("{}", s))?
        .into_iter()
        .collect_tuple()
        .ok_or(format!("failed to parse input line: {}", p))
}

pub(crate) fn result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let paths: Result<Vec<Vec<(i32, i32)>>, String> = lines
        .iter()
        .map(|l| {
            l.split(" -> ")
                .map(parse_point)
                .collect::<Result<Vec<(i32, i32)>, String>>()
        })
        .collect();

    // println!("input: {:?}", paths);
 
    let mut c: Cave = Cave::new(paths?);

    loop {
        // println!("next sand enters");
        let s = c.move_sand();
        // print!("\x1B[2J");  // clear terminal
        //  println!("{}", c);

        if  s == (500,0) {
            println!("breaking");
            break;
        }
    }

    //println!("{}", c);
    println!("Part1 result: {}", c.sand_unit_total);

    Ok(())
}
