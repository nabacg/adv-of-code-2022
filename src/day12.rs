use core::fmt;
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Vertex {
    i: usize,
    letter: char,
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.letter, self.i)
    }
}

impl Vertex {
    fn new(i: usize, letter: char) -> Vertex {
        Vertex { i, letter }
    }

    fn height(&self) -> i32 {
        match self.letter {
            'S' => 'a' as i32,
            'E' => 'z' as i32,
            _ => self.letter as i32,
        }
    }
    fn height_diff(&self, other: &Vertex) -> i32 {
        other.height() - self.height()
    }
}

struct Graph {
    adjacency_list: HashMap<Vertex, Vec<(Vertex, usize)>>,
    start: Vertex,
    target: Vertex,
}

impl Graph {
    fn adjacent_nodes(vs: &Vec<Vertex>, v_idx: usize, cols: usize) -> Vec<(Vertex, usize)> {
        let v = &vs[v_idx];
        let vs_length = vs.len();
        let vx = v_idx % cols;
        let vy = v_idx / cols;
        let max_rows = vs_length / cols;

        let ns = vec![(vx as i32) - 1, vx as i32, (vx as i32) + 1]
            .into_iter()
            .filter(|&x| x >= 0)
            .map(|x| x as usize)
            .filter(|&x| x < cols)
            .flat_map(|x| {
                // println!("x:{}", x);
                vec![(vy as i32) - 1, vy as i32, (vy as i32) + 1]
                    .into_iter()
                    .filter(|&y| y >= 0)
                    .map(|y| y as usize)
                    .filter(|&y| y < max_rows)
                    .map(move |y| (x, y))
                    .filter(|(x, y)| !(x == &vx && y == &vy))
                    .filter(|(x, y)| x.abs_diff(vx) + y.abs_diff(vy) == 1)
                    .map(|(x, y)| {
                        // println!("({}, {}) -> {}", x, y,  x + y*self.cols);
                        let new_v = vs[x + y * cols];
                        let height_diff = v.height_diff(&new_v);
                        (new_v, height_diff)
                    })
                    .filter(|(_, hdiff)| *hdiff < 2)
                    .map(|(v, _)| (v, 1)) // distance is always 1, if height differnce is at most 1 higher
            })
            .collect_vec();

        ns
    }
}

fn new(ls: String) -> Result<Graph, String> {
    let cols = ls
        .find('\n')
        .ok_or("Invalid input - single line tree grid is not a grid")?;
    let vertices = ls
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(i, c)| Vertex::new(i, c))
        .collect::<Vec<Vertex>>();

    let adjacency_list: HashMap<Vertex, Vec<(Vertex, usize)>> = vertices
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let ns = Graph::adjacent_nodes(&vertices, i, cols);
            (v, ns)
        })
        .collect();

    let &target = vertices
        .iter()
        .filter(|&v| v.letter == 'E')
        .take(1)
        .next()
        .ok_or("Cannot find target node index, i.e. grid field - 'E'")?;

    let &start = vertices
        .iter()
        .filter(|v| v.letter == 'S')
        .take(1)
        .next()
        .ok_or("Cannot find target node index, i.e. grid field - 'E'")?;

    Ok(Graph {
        adjacency_list,
        start,
        target,
    })
}

struct PathDistance {
    target: Vertex,
    distance: usize,
}
impl PathDistance {
    fn new(v: Vertex, d: usize) -> PathDistance {
        PathDistance {
            target: v,
            distance: d,
        }
    }
}

impl Ord for PathDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // since we'll be looking for MIN distance, need to swap around self and other
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for PathDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl PartialEq for PathDistance {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target && self.distance == other.distance
    }
}

impl Eq for PathDistance {}

fn dijkstra(g: &Graph) -> HashMap<Vertex, usize> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(g.start, 0);
    to_visit.push(PathDistance::new(g.start, 0));

    while let Some(PathDistance { target, distance }) = to_visit.pop() {
        if target == g.target {
            return distances;
        }

        if !visited.insert(target) {
            // if insert failed, we already visited this node
            continue;
        }

        if let Some(neighbours) = g.adjacency_list.get(&target) {
            for (neighbour, n_distance) in neighbours {
                let new_distance = distance + n_distance;

                let is_shorter = distances
                    .get(&neighbour)
                    .map_or(true, |&current_dist| new_distance < current_dist);

                if is_shorter {
                    distances.insert(*neighbour, new_distance);
                    to_visit.push(PathDistance::new(*neighbour, new_distance));
                }
            }
        }
    }

    distances
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
    let ns = g.adjacency_list.get(&g.target);
    // let to_target2 = g.adjacency_list.get(&Vertex::new(3475, 'y'));
    let distances = dijkstra(&g);

    let target_distance = distances.get(&g.target);
    println!("distance:{:?}", target_distance);

    Ok(())
}
