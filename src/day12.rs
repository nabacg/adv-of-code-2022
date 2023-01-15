use core::fmt;
use std::{error::Error, collections::{HashMap, BinaryHeap, HashSet}};
use std::cmp::Reverse;
use itertools::Itertools;



struct Vertex {
    i: usize,
    x: usize,
    y: usize,
    letter: char,
    cols: usize,
}


impl Vertex {
    fn new(i: usize, letter: char, cols: usize) -> Vertex {
        let x = i % cols;
        let y = i / cols;
        Vertex { i, x, y, letter, cols, }
    }

    fn height(&self) -> usize {
        return self.letter as usize
    }

    fn is_neighbour(&self, other: &Vertex) -> bool {
        self.x.abs_diff(other.x) <= 1 &&
         self.y.abs_diff(other.y) <= 1
    }

    fn neighbours(&self, full_length: usize) -> Vec<usize> {
        let max_rows = full_length / self.cols;
        vec![(self.x as i32) -1, self.x as i32, (self.x as i32) +1]
        .into_iter()
        .filter(|&x| x >= 0)
        .map(|x| x as usize)
            .filter(|&x|  x < self.cols )
            .flat_map(|x| {
                // println!("x:{}", x);
                vec![(self.y as i32)-1, self.y as i32, (self.y as i32)+1]                    
                    .into_iter()
                    .filter(|&y| y >= 0)
                    .map(|y| y as usize)
                    .filter(|&y|  y < max_rows)                    
                    .map(move |y| (x,y))
                    .filter(|(x, y)| !(x == &self.x && y == &self.y))
                    .filter(|(x,y)| x.abs_diff(self.x)+y.abs_diff(self.y) == 1)
                    .map(|(x,y)| {
                        // println!("({}, {}) -> {}", x, y,  x + y*self.cols);
                        x + y*self.cols
                    })
            }).collect_vec()
    }
}

struct Graph {
    vertices: Vec<Vertex>,
    target_index: usize,
    // target: &'a Vertex,
    start_index: usize,
    // start: &'a Vertex,
}

impl Graph {
    fn distance(a: &Vertex, b: &Vertex) -> usize {
        if !a.is_neighbour(b) {
            usize::MAX
        } else if a.height().abs_diff(b.height()) > 1 {
            1000
        } else {
            1
        }
    }
}

fn new(ls: String) -> Result<Graph, String> {
    let cols = ls.find('\n').ok_or("Invalid input - single line tree grid is not a grid")?;
    let vertices = 
    ls.chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(i, c)| Vertex::new(i, c, cols) )
        .collect::<Vec<Vertex>>();

    let (target_index, target) = vertices.iter()
                .enumerate()
                .filter(|(i, v)| v.letter == 'E')
                .take(1)
                .next()
                .ok_or("Cannot find target node index, i.e. grid field - 'E'")?;

    let (start_index, start) = vertices.iter()
                .enumerate()
                .filter(|(i, v)| v.letter == 'E')
                .take(1)
                .next()
                .ok_or("Cannot find target node index, i.e. grid field - 'E'")?;

    Ok(Graph{ 
        vertices,
        start_index,
        // start,
        target_index,
        // target,
    })
}

#[derive(Eq)]
struct Distance {
    i: usize,
    distance: usize,
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        Some(other.distance.cmp(&self.distance))
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
fn dijkstra(g: &Graph) -> Vec<usize> {
    let mut result_path = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    
    result_path.insert(g.start_index, 0);
    to_visit.push((g.start_index, 0));

    let vertices_len = g.vertices.len();

    while let Some((v_index, v_distance)) = to_visit.pop() {
            if !visited.insert(v_index) {
                // if insert failed, we already visited this node
                continue;
            }

            // TODO this is akward to deal with, move away from dealing with indices, to dealing with Vertex (would mean implementing Copy on it)
            let v = g.vertices.get(v_index).expect("can't get current v_index!");
            

            let neighbourhood_distances: Vec<(&Vertex, usize)> = v.neighbours(vertices_len).iter().flat_map(|&n| 
                if let Some(n) = g.vertices.get(n) {
                    vec![(n, Graph::distance(v, n))]
                } else {
                    vec![]
                }).collect();
                
            for (neighbour, distance) in neighbourhood_distances {
                let new_distance = v_distance + distance;

                let is_shorter = result_path.get(&neighbour.i)
                .map_or(true, |&current_dist| new_distance < current_dist);
            
                if is_shorter {
                    result_path.insert(neighbour.i, new_distance);
                    to_visit.push((neighbour.i, new_distance));
            }


        };
    }
    result_path.keys().map(|&i| i).collect_vec()
}
//https://codereview.stackexchange.com/a/202879

pub(crate) fn result(input: String) -> Result<(), Box<dyn Error>> {
    let v = Vertex::new(39, 'S', 8);
    let v = Vertex{
        i: 4, x: 0, y: 4, letter: 'a', cols: 8,
    };
    let ns = v.neighbours(40);

    let g = new(input)?;
    let path = dijkstra(&g);



    println!("path:{:?}", path.iter().rev());

    Ok(())
}