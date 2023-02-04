use std::{error::Error, num::ParseIntError};

// DAY 4
#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn from_input(i: &str) -> Result<Section, Box<dyn Error>> {
        let edges: Result<Vec<u32>, ParseIntError> =
            i.split("-").map(|e| e.parse::<u32>()).collect();
        let edges = edges?;
        if edges.len() != 2 {
            Err(format!(
                "Invalid input, expected 2 part assigment separated by ',', got: {}",
                i
            )
            .into())
        } else {
            Ok(Section {
                start: edges[0],
                end: edges[1],
            })
        }
    }

    fn left_overlap(&self, another: &Section) -> bool {
        another.end >= self.start && another.start <= self.end
    }

    fn overlap(&self, another: &Section) -> bool {
        // left overlap
        // ...456...  4-6
        // .234.....  2-4
        self.left_overlap(another) ||
        //right overlap
        // .234.....  2-4
        // ...456...  4-6
         another.left_overlap(self)
    }
}

#[derive(Debug)]
struct CleaningAssignment {
    left: Section,
    right: Section,
}

impl CleaningAssignment {
    fn from_input(l: &String) -> Result<CleaningAssignment, Box<dyn Error>> {
        let parts: Vec<&str> = l.split(",").collect();
        if parts.len() != 2 {
            Err(format!(
                "Invalid input, expected 2 part assigment separated by ',', got: {}",
                l
            )
            .into())
        } else {
            let l = Section::from_input(parts[0])?;
            let r = Section::from_input(parts[1])?;
            Ok(CleaningAssignment { left: l, right: r })
        }
    }

    fn pair_overlap(&self) -> bool {
        self.left.overlap(&self.right)
    }
}

pub fn result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let overlapping: Result<Vec<CleaningAssignment>, Box<dyn Error>> =
        lines.iter().map(CleaningAssignment::from_input).collect();
    let overlapping_count = overlapping?
        .iter()
        .filter(|a| a.pair_overlap())
        .map(|i| i)
        .count();
    println!("Result is: {}", overlapping_count);

    Ok(())
}

// DAY 4 END
