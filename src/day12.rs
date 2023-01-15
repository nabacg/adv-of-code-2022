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
        match self.letter {
            'S' => 'a' as usize,
            'E' => 'z' as usize, 
            _ => self.letter as usize
        }
    }
    fn distance(&self, other: &Vertex) -> usize {
        self.height().abs_diff(other.height())
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

    fn adjacent_nodes(vs:&Vec<Vertex>, v: &Vertex, v_idx: usize, vs_length: usize, cols: usize) -> Vec<(Vertex, usize)> {
        let vx = v_idx % cols;
        let vy = v_idx / cols;
        let max_rows = vs_length / cols;

        let ns =
        vec![(vx as i32) -1, vx as i32, (vx as i32) +1]
        .into_iter()
        .filter(|&x| x >= 0)
        .map(|x| x as usize)
            .filter(|&x|  x < cols )
            .flat_map(|x| {
                // println!("x:{}", x);
                vec![(vy as i32)-1, vy as i32, (vy as i32)+1]                    
                    .into_iter()
                    .filter(|&y| y >= 0)
                    .map(|y| y as usize)
                    .filter(|&y|  y < max_rows)                    
                    .map(move |y| (x,y))
                    .filter(|(x, y)| !(x == &vx && y == &vy))
                    .filter(|(x,y)| x.abs_diff(vx)+y.abs_diff(vy) == 1)                
                    .map(|(x,y)| {
                        // println!("({}, {}) -> {}", x, y,  x + y*self.cols);
                        let n_v = &vs[x + y*cols];
                        let new_v = Vertex {
                            letter: n_v.letter,
                            i: x + y*cols,
                            x,
                            y,
                            cols,
                        };
                        let dist  = v.height().abs_diff(new_v.height()); //Graph::distance(v, &new_v);
                        (new_v, dist)
                    })                    
                    .filter(|(_, dist)| *dist < 2)
            }).collect_vec();

         ns   
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

    let vs_len = vertices.len();

    let adjacency_list:Vec<(&Vertex, Vec<(Vertex, usize)>)> = vertices
        .iter()
        .enumerate()
        .map(|(i, v) | {
        let ns = Graph::adjacent_nodes(&vertices, v,i, vs_len, cols);
        (v, ns)
    }).collect();

    let (target_index, target) = vertices.iter()
                .enumerate()
                .filter(|(i, v)| v.letter == 'E')
                .take(1)
                .next()
                .ok_or("Cannot find target node index, i.e. grid field - 'E'")?;

    let (start_index, start) = vertices.iter()
                .enumerate()
                .filter(|(i, v)| v.letter == 'S')
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


    // let v = Vertex::new(39, 'l', 8);
    // let v2=  Vertex {
    //     i: 4, x: 0, y: 4, letter: 'm', cols: 8,
    // };
    // let h = v.height();
    // let h2 = v2.height();

    // let dist = v.distance(&v2);
    // let ns = v.neighbours(40);

    let g = new(input)?;
    let path = dijkstra(&g);



    println!("path:{:?}", path.iter().rev());

    Ok(())
}