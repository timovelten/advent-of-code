use crate::{
    util::{Edge, Graph},
};

const INPUT: &str = include_str!("../input/day15");

fn checked_add(a: usize, b: isize) -> Option<usize> {
    if b >= 0 {
        a.checked_add(b as usize)
    } else {
        a.checked_sub(-b as usize)
    }
}

fn in_bounds(
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    width: usize,
    height: usize,
) -> Option<(usize, usize)> {
    let nx = checked_add(x, dx)?;
    let ny = checked_add(y, dy)?;

    if nx < width && ny < height {
        Some((nx, ny))
    } else {
        None
    }
}

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines();

    let mut grid = Vec::new();
    let mut width = 0;

    for line in lines {
        if width == 0 {
            width = line.len();
        }

        for c in line.chars() {
            grid.push(c.to_digit(10).ok_or(crate::Error::InvalidInput)? as u8);
        }
    }

    let width = width;
    let height = grid.len() / width;

    struct GridGraph<F: Fn(usize, usize) -> u64> {
        cost: F,
        width: usize,
        height: usize,
    }

    impl <F: Fn(usize, usize) -> u64> Graph<usize> for GridGraph<F> {
        fn neighbors(&self, &v: &usize) -> Vec<Edge<usize>> {
            let mut ret = Vec::new();
            let (x, y) = (v % self.width, v / self.width);
            
            for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some((nx, ny)) = in_bounds(x, y, dx, dy, self.width, self.height) {
                    let vertex = nx + ny * self.width;
                    let cost = (self.cost)(nx, ny);

                    ret.push(Edge { vertex, cost });
                }
            }

            ret
        }
    }

    // Part 1
    let graph1 = GridGraph {
        cost: (|x, y| grid[x + y * width] as u64),
        width, height
    };

    let end1 = graph1.width * graph1.height - 1;
    let (dist1, _) =  graph1.dijsktra(0, Some(end1), false);
    println!("Problem 1: {}", dist1[&end1]);

    // Part 2

    let modified_cost = |x: usize, y: usize| {
        let base_cost = grid[(x % width) + (y % height) * width] as u64;
        let i = (x / width) as u64 + (y / height) as u64;
        (base_cost - 1 + i) % 9 + 1
    };
    let graph2 = GridGraph {
        cost: modified_cost,
        width: 5 * width,
        height: 5 * height
    };

    let end2 = graph2.width * graph2.height - 1;
    let (dist2, _) =  graph2.dijsktra(0, Some(end2), false);
    println!("Problem 2: {}", dist2[&end2]);

    Ok(())
}
