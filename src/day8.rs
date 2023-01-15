use core::fmt;
use std::{error::Error, num::ParseIntError, ops::Index};

// thanks https://stackoverflow.com/a/70511530
#[derive(Debug)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Vec2d<T> {
    pub fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
        assert!(vec.len() == row * col);
        Self { vec, row, col }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.col * row;
        &self.vec[i..(i + self.col)]
    }

    pub fn index(&self, row: usize, col: usize) -> &T {
        let i = self.col * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&mut self, row: usize, col: usize) -> &mut T {
        let i = self.col * row;
        &mut self.vec[i + col]
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.row {
            if i != 0 {
                str.push_str(", ");
            }
            str.push_str(&format!("{:?}", &self.row(i)));
        }
        write!(f, "[{}]", str)
    }
}

struct TreeGrid {
    tree_heights: Vec2d<u32>,
}

impl TreeGrid {
    fn new(ls: String) -> Result<TreeGrid, String> {
        const RADIX: u32 = 10;
        let cols = ls
            .find('\n')
            .ok_or("Invalid input - single line tree grid is not a grid")?;
        let rows = ls.chars().filter(|&c| c == '\n').count() + 1;
        let data: Result<Vec<u32>, String> = ls
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| {
                c.to_digit(RADIX)
                    .ok_or(format!("failed to parse char: {}", c))
            })
            .collect::<Result<Vec<u32>, String>>();

        Ok(TreeGrid {
            tree_heights: Vec2d::new(data?, rows, cols),
        })
    }

    fn inner_trees(&self) -> Vec<TreeCoord> {
        (1..self.tree_heights.col - 1)
            .into_iter()
            .flat_map(|col| {
                (1..self.tree_heights.row - 1)
                    .map(|r| TreeCoord { x: r, y: col })
                    .collect::<Vec<TreeCoord>>()
            })
            .collect()
    }

    fn height(&self, c: &TreeCoord) -> &u32 {
        self.tree_heights.index(c.x, c.y)
    }

    fn edge_tree_count(&self) -> usize {
        2 * self.tree_heights.col + 2 * (self.tree_heights.row - 2) // 2 full col lengths plus 2 * row lengths - 2( to prevent double counting corners ) )
    }
}

#[derive(Debug)]
struct TreeCoord {
    x: usize,
    y: usize,
}

impl fmt::Display for TreeCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl TreeCoord {
    fn is_visible(&self, tg: &TreeGrid) -> bool {
        let this_height = tg.height(self);
        let right = VizPath::right(self, tg.tree_heights.col);
        let left = VizPath::left(self);
        let up = VizPath::up(self);
        let down = VizPath::down(self, tg.tree_heights.row);

        !vec![left, up, right, down].iter().all(|p| {
            let covered = p
                .path
                .iter()
                .map(|c| tg.height(c))
                .any(|h| h >= this_height);
            // println!("Tree {} is {} visible from {}", self, if covered { "Not"} else {""}, p.dir);
            covered
        })
    }

    fn scenic_score(&self, tg: &TreeGrid) -> usize {
        let this_height = tg.height(self);
        let right = VizPath::right(self, tg.tree_heights.col);
        let left = VizPath::left(self);
        let up = VizPath::up(self);
        let down = VizPath::down(self, tg.tree_heights.row);

        vec![left, up, right, down]
            .iter()
            .map(|p| {
                let scenic_path = p
                    .path
                    .iter()
                    .take_while(|&c| {
                        let is_lower = tg.height(c) < this_height;
                        // println!("\t{}, with height: {} is lower? {} than {}", c, tg.height(c), is_lower, this_height);
                        is_lower
                    })
                    .collect::<Vec<&TreeCoord>>();
                let unobscuring_tree_count = scenic_path.iter().count();

                let scenic_count = if unobscuring_tree_count == p.path.len() {
                    // unobscured view till edge
                    unobscuring_tree_count
                } else {
                    // we need to add the 1 tree that obscured the view
                    unobscuring_tree_count + 1
                };
                println!(
                    "Tree: {} has scenic count of: {} from {}",
                    self, scenic_count, p.dir
                );
                // if scenic_count == 0 {
                //     1
                // } else {
                //     scenic_count
                // }
                scenic_count
            })
            .fold(1, std::ops::Mul::mul)
    }
}

struct VizPath {
    path: Vec<TreeCoord>,
    dir: String,
}

impl VizPath {
    fn down(c: &TreeCoord, grid_rows: usize) -> VizPath {
        VizPath {
            dir: "DOWN".to_string(),
            path: (c.x + 1..grid_rows)
                .map(|r| TreeCoord { x: r, y: c.y })
                .collect(),
        }
    }

    fn up(c: &TreeCoord) -> VizPath {
        VizPath {
            dir: "UP".to_string(),
            path: (0..c.x)
                .rev() // it's easier if path Coords always go from TreeCord to the edge
                .map(|r| TreeCoord { x: r, y: c.y })
                .collect(),
        }
    }

    fn right(c: &TreeCoord, grid_cols: usize) -> VizPath {
        VizPath {
            dir: "RIGHT".to_string(),
            path: (c.y + 1..grid_cols)
                .map(|col| TreeCoord { x: c.x, y: col })
                .collect(),
        }
    }
    fn left(c: &TreeCoord) -> VizPath {
        VizPath {
            dir: "LEFT".to_string(),
            path: (0..c.y)
                .rev() // it's easier if path Coords always go from TreeCord to the edge
                .map(|col| TreeCoord { x: c.x, y: col })
                .collect(),
        }
    }
}

pub fn result(input: String) -> Result<(), Box<dyn Error>> {
    let tg = TreeGrid::new(input)?;
    let inner_ts = tg.inner_trees();
    let visible_inner_trees: Vec<&TreeCoord> =
        inner_ts.iter().filter(|&tc| tc.is_visible(&tg)).collect();
    // println!("{:?}", visible_inner_trees);
    let inner_count = visible_inner_trees.len();
    println!("part1 - result: {}", inner_count + tg.edge_tree_count());

    let most_scenic_tree = inner_ts
        .iter()
        .max_by(|a, b| a.scenic_score(&tg).cmp(&b.scenic_score(&tg)))
        .ok_or("cannot find max_scenic_score, empty inner trees ?")?;

    println!(
        "part2 - result: {}, tree: {}",
        most_scenic_tree.scenic_score(&tg),
        most_scenic_tree
    );
    Ok(())

    // // let test = TreeCoord{ x:1, y: 2};
    // println!("part2 - result: {}, tree: {}", test.scenic_score(&tg), test);
    // Ok(())
}
