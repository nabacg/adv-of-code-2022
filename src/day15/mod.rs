use std::{error::Error, num::ParseIntError, collections::HashSet};
use itertools::Itertools;
use regex::Regex;


type Coord = (i32, i32);


#[derive(Debug)]
struct Sensor {
    pos: Coord,
    closest_beacon: Coord,
    covered_range: u32,
} 

#[cfg(test)]
mod sensor_test {
    use super::Sensor;


    #[test]
    fn sensor_perimiter_test_range_zero() {
        let s = Sensor::new((0,0), (0,0));

        let expected_perimiter = vec![(0,-1), (1,0), (0,1), (-1,0)];
        assert_eq!(expected_perimiter, s.sensor_perimiter(-2, 2));
    }

    #[test]
    fn sensor_perimiter_test_range_one() {
        let s = Sensor::new((0,0), (0,1));

        let expected_perimiter = vec![(0,-2), (1, -1), (2,0), (1,1), (0,2), (-1,1), (-2, 0), (-1, -1)];
        assert_eq!(expected_perimiter, s.sensor_perimiter(-2, 2));
    }

    #[test]
    fn sensor_perimiter_test_range_one_with_bounds() {
        let s = Sensor::new((0,0), (0,1));

        let expected_perimiter = vec![(2,0), (1,1), (0,2)];
        assert_eq!(expected_perimiter, s.sensor_perimiter(0, 2));
    }
}

impl Sensor {
    //return list of Coords just outside Sensor's range
    fn sensor_perimiter(&self, lower_bound: i32, upper_bound: i32) -> Vec<Coord> {
        let (s_x, s_y) = self.pos;
        let perimiter_range = (self.covered_range as i32) + 1;
        let min_x = s_x-perimiter_range;
        let max_x = s_x + perimiter_range;
        let top_descending = (s_x..max_x+1).zip((s_y-perimiter_range)..s_y+1).collect_vec();
        // println!("{:?}", top_descending);
        let bottom_rising = (s_x..max_x).rev().zip((s_y+1)..(s_y+perimiter_range+1)).collect_vec();
        // println!("{:?}", bottom_rising);
        let bottom_descending =  (min_x..s_x).zip((s_y)..(s_y+perimiter_range)).collect_vec();
        // println!("{:?}", bottom_descending);
        let top_rising = ((min_x+1)..s_x).zip(((s_y-perimiter_range)..s_y).rev()).collect_vec();
        // println!("{:?}", top_rising);

        top_descending.into_iter()
            .chain(bottom_rising.into_iter())
            .chain(bottom_descending.into_iter())
            .chain(top_rising.into_iter())
            .filter(|(x,y)| lower_bound <= *x && x <= &upper_bound 
                                && lower_bound <= *y && y <= &upper_bound)
            .collect()
    }

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
    
    // println!("min_x:{min_x}, max_x:{max_x}");
    // let y = 2000000;

    // let beacon_set = HashSet::<_>::from_iter(sensors.iter().map(|s| s.closest_beacon));

    // let res: usize = (min_x..max_x+1).map(|x| (x, y)).filter(|&c| {
    //     let covered =    sensors.iter().any(|s| s.is_within_range(c));
    //     // println!("({},{}) is {}", c.0, c.1, covered);
    //     covered
    // })
    //  .filter(|c| !beacon_set.contains(c))
    // .count();

    // println!("Part1 Result: {}", res);


    let distr_max = 4000000;
    // https://github.com/saulvaldelvira/AdventOfCode/blob/5ebda5ec175e15bdb42f217767592e68ae00a829/2022/Day15/puzzle15.c#L165


    // let is_it = sensors
    // .iter()
    // .filter(|s| s.is_within_range((14,11)))
    // .map(|s| (s, Sensor::manhattan_distance(s.pos, (14,11)), s.covered_range))
    // .collect_vec();

    // let all_perimiter = sensors.iter()
    //     .map(|s| (s.pos, s.covered_range, s.sensor_perimiter(0, distr_max)));
    //     //.for_each(|d| println!("{d:?}", ));

        
    // let ((s_x, s_y), r, perimiter) = &all_perimiter.collect_vec()[0];    

    // let perimit_set = HashSet::<_>::from_iter(perimiter);

    // let max_grid = 4000000;
    // (0..max_grid).map(|y| {
    //     (0..max_grid).map(|x| match (x,y) {
    //         (x,y) if &x == s_x && &y == s_y => "S",
    //         p if perimit_set.contains(&p) => "#",
    //         _ => " "
    //     }).join("")
    // }).for_each(|l| println!("{l}"));

    if let Some(distress_beacon) = sensors
        .iter()
        .map(|s|s.sensor_perimiter(0,distr_max)
                            .iter()
                            .skip_while(|&c| {
                                let covered = sensors
                                                        .iter()
                                                        .any(|s| s.is_within_range(*c));
                                //  println!("({},{}) is {}", c.0, c.1, covered);
                                                        covered 
                            })
                            .map(|&c| c)
                            .next())
        .skip_while(|s| s.is_none())
        .map(|c| c.unwrap())
        .next() {
        println!("Part2 Distress Beacon coords: {:?}", distress_beacon);
        let (x, y) = distress_beacon;
        let res_freq: i64 = (x as i64) * 4000000 + (y as i64);
        println!("Part2 Result: {res_freq}");
    } else {
        println!("not found")
    }


    Ok(())
}