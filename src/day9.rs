use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

#[derive(Debug)]
enum Dir {
    NoOp,
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl Dir {
    fn new(d: &str) -> Result<Dir, String> {
        match d {
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(format!(
                "Invalid input, expected one of U | D | L | R, got: {} ",
                d
            )),
        }
    }
}

struct Move {
    dir: Dir,
    steps: u16,
}

impl Move {
    fn new(line: String) -> Result<Move, String> {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 2 {
            Err(format!("Invalid input, expected whitespace separated 2 part line, Dir and Num_of_steps, got {}", line))
        } else {
            let dir = Dir::new(parts[0])?;
            let steps = parts[1].parse::<u16>().map_err(|e| format!("{}", e))?;
            Ok(Move {
                dir: dir,
                steps: steps,
            })
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Knot {
    x: i32,
    y: i32,
}

impl fmt::Display for Knot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Knot {
    fn new(x: i32, y: i32) -> Knot {
        Knot { x: x, y: y }
    }

    fn move_to(&self, d: &Dir) -> Knot {
        match d {
            Dir::Up => Knot::new(self.x, self.y + 1),
            Dir::Down => Knot::new(self.x, self.y - 1),
            Dir::Left => Knot::new(self.x - 1, self.y),
            Dir::Right => Knot::new(self.x + 1, self.y),
            Dir::NoOp => *self,
            Dir::LeftUp => Knot::new(self.x - 1, self.y + 1),
            Dir::RightUp => Knot::new(self.x + 1, self.y + 1),
            Dir::LeftDown => Knot::new(self.x - 1, self.y - 1),
            Dir::RightDown => Knot::new(self.x + 1, self.y - 1),
        }
    }

    fn is_adjacent(&self, other: &Knot) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    fn eq_row(&self, other: &Knot) -> bool {
        self.x == other.x
    }

    fn eq_col(&self, other: &Knot) -> bool {
        self.y == other.y
    }

    fn is_above(&self, other: &Knot) -> bool {
        other.y > self.y
    }

    fn is_below(&self, other: &Knot) -> bool {
        other.y < self.y
    }

    fn is_left(&self, other: &Knot) -> bool {
        other.x < self.x
    }

    fn is_right(&self, other: &Knot) -> bool {
        other.x > self.x
    }

    fn dir_towards(&self, other: &Knot) -> Dir {
        match (self, other) {
            (a, b) if a.eq_col(b) && a.eq_row(b) => Dir::NoOp,
            (a, b) if a.is_left(b) && a.is_above(b) => Dir::LeftUp,
            (a, b) if a.is_right(b) && a.is_above(b) => Dir::RightUp,
            (a, b) if a.is_left(b) && a.is_below(b) => Dir::LeftDown,
            (a, b) if a.is_right(b) && a.is_below(b) => Dir::RightDown,
            (a, b) if a.is_above(b) => Dir::Up,
            (a, b) if a.is_below(b) => Dir::Down,
            (a, b) if a.is_left(b) => Dir::Left,
            (a, b) if a.is_right(b) => Dir::Right,

            _ => unreachable!(),
        }
    }
}

struct Rope {
    // head: Knot,
    // tail: Knot,
    knots: Vec<Knot>,
    end_knot_history: Vec<Knot>,
    end_knot_index: usize,
}

impl Rope {
    fn new(knots_count: usize) -> Rope {
        Rope {
            knots: (0..knots_count).map(|_| Knot::new(0, 0)).collect(),
            end_knot_index: knots_count - 1,
            end_knot_history: vec![],
        }
    }

    fn head(&self) -> &Knot {
        &self.knots[0]
    }

    fn tail(&self) -> &Knot {
        &self.knots[self.end_knot_index]
    }

    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.steps {
            self.apply_dir(&m.dir);
        }
    }

    fn apply_dir(&mut self, dir: &Dir) {
        println!("{}", self);
        let new_head = self.knots[0].move_to(dir);

        let mut knots = vec![new_head];
        let _new_knots = self.knots[1..].iter().fold(&mut knots, |acc, k| {
            let head = acc
                .last()
                .expect("Should never be empty, we start from [1..]");
            let tail = k;
            println!("Head: {}, Tail: {}", head, tail);
            if !head.is_adjacent(tail) {
                let tail_move = tail.dir_towards(&head);
                println!("Tail Move: {:?}", tail_move);
                let new_tail = tail.move_to(&tail_move);
                acc.push(new_tail);
                acc
            } else {
                acc.push(*k);
                acc
            }
        });
        self.end_knot_history.push(self.tail().clone());
        self.knots = knots;
    }


    fn print_tail_history(&self) {
        let head = self.head();
        let tail = self.tail();
        let max_rows = head.x.max(tail.x).max(5);
        let max_cols = head.y.max(tail.y).max(6);

        (0..max_rows).rev().for_each(|y| {
            let row_str: Vec<&str> = (0..max_cols)
                .map(|x| {
                    if x == 0 && y == 0 {
                        "s"
                    } else if self.end_knot_history.contains(&Knot::new(x, y)) {
                        "#"
                    } else {
                        "."
                    }
                })
                .collect();
            println!("{}", row_str.join(""));
        });
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let head = self.head();
        let tail = self.tail();
        writeln!(f, "Head: {}, Tail: {}", head, tail)?;
        let max_cols = self.knots.iter().map(|k| k.x).max().unwrap();
        let max_rows = self.knots.iter().map(|k| k.y).max().unwrap();

        let max_cols = max_cols.max(6);
        let max_rows = max_rows.max(5);

        let knots_lookup: HashMap<(i32, i32), String> =
            HashMap::from_iter(self.knots.iter().enumerate().map(|(i, k)| {
                if i == 0 {
                    ((k.x, k.y), "H".to_string())
                } else {
                    ((k.x, k.y), i.to_string())
                }
            }));

        (0..max_rows).rev().for_each(|y| {
            let row_str: Vec<&str> = (0..max_cols)
                .map(|x| {
                    if knots_lookup.contains_key(&(x, y)) {
                        knots_lookup.get(&(x, y)).unwrap()
                    } else {
                        "."
                    }
                })
                .collect();
            writeln!(f, "{}", row_str.join("")).expect("Failed to writeln! row_str");
        });
        writeln!(f, "")
    }
}

pub fn result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let moves: Result<Vec<Move>, String> = lines.into_iter().map(Move::new).collect();
    let mut r = Rope::new(10);

    moves?.iter().for_each(|m| r.apply_move(m));
    r.end_knot_history.push(r.tail().clone()); // don't forget to add last tail position to history
    r.print_tail_history();
    let unique_tail_coords = HashSet::<_>::from_iter(r.end_knot_history);
    println!("Part1 Result: {}", unique_tail_coords.len());
    Ok(())
}
