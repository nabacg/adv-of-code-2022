use std::{error::Error, num::ParseIntError, collections::HashSet};
use regex::Regex;


type Coord = (i32, i32);


#[derive(Debug)]
struct Sensor {
    pos: Coord,
    closest_beacon: Coord,
    covered_range: u32,
} 

impl Sensor {


    fn is_within_range(&self, c: Coord) -> bool {
        Sensor::manhattan_distance(self.pos, c) <= self.covered_range
    } 

    fn new(pos: Coord, closest_beacon: Coord) -> Sensor {
        let covered_range = Sensor::manhattan_distance(pos, closest_beacon);
        Sensor { pos, closest_beacon, covered_range }
    }

    
    fn parse(ls: Vec<String>) -> Result<Vec<Sensor>, ParseIntError> {
        let coord_re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

        ls.into_iter().map(|l| {
            let cords:Result<Vec<Coord>, ParseIntError> =
            coord_re
            .captures_iter(&l).map(|cap| 
                Ok((cap[1].parse::<i32>()?, cap[2].parse::<i32>()?)))
            .collect();
            let cords = cords?;
            Ok(Sensor::new(cords[0], cords[1]))
        }).collect()
    }

    fn manhattan_distance((x_a, y_a): Coord, (x_b, y_b): Coord) -> u32 {
        x_b.abs_diff(x_a) + y_b.abs_diff(y_a)
    }
}

pub(crate) fn result(ls: Vec<String>) -> Result<(), Box<dyn Error>> {
    let sensors: Result<Vec<Sensor>, ParseIntError> = Sensor::parse(ls);
    let sensors = sensors?;
    // println!("{:?}", sensors);

    // check number of fields covered in y=10
    

    let min_x = sensors.iter().map(|s| s.pos.0 - (s.covered_range as i32)).min().ok_or("empty sensor list in min_x")?;
    let max_x = sensors.iter().map(|s| s.pos.0 + (s.covered_range as i32)).max().ok_or("empty sensor list in max_x")?;
    
    println!("min_x:{}, max_x:{}", min_x, max_x);
    let y = 2000000;

    let beacon_set = HashSet::<_>::from_iter(sensors.iter().map(|s| s.closest_beacon));

    let res: usize = (min_x..max_x+1).map(|x| (x, y)).filter(|&c| {
        let covered =    sensors.iter().any(|s| s.is_within_range(c));
        // println!("({},{}) is {}", c.0, c.1, covered);
        covered
    })
     .filter(|c| !beacon_set.contains(c))
    .count();

    // 4208196 is too low.
    // 5046501 is too high
    println!("Part1 Result: {}", res);

    Ok(())
}