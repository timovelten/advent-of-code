use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day03.txt");

pub fn solve() -> crate::Result<()> {
    let mut graph = Graph::new();

    for (y, line) in INPUT.lines().enumerate() {
        let mut vertex_in_progress = None;

        for (x, c) in line.char_indices() {
            if let Some(d) = c.to_digit(10) {
                // We have found a digit. Defer inserting it until we run off the end of the line.
                if let Some(Vertex::Number { xlen, num, .. }) = &mut vertex_in_progress {
                    *num = *num * 10 + (d as i64);
                    *xlen += 1;
                } else {
                    vertex_in_progress = Some(Vertex::Number {
                        num: d as i64,
                        y: y as i32,
                        x: x as i32,
                        xlen: 1,
                    });
                }
            } else {
                // No digit. Commit the last number if there is one.
                if let Some(v) = vertex_in_progress.take() {
                    graph.insert(v);
                }

                if c != '.' {
                    // We have found another symbol, insert it.
                    graph.insert(Vertex::Symbol {
                        c,
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }

        // We may end with a number..
        if let Some(v) = vertex_in_progress.take() {
            graph.insert(v);
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for i in 0..graph.vertices.len() {
        match &graph.vertices[i] {
            Vertex::Number {
                num,
                y: _,
                x: _,
                xlen: _,
            } => {
                let mut adjacent_symbols = 0;
                graph.for_each_neighbor(i, |w| {
                    if let Some(Vertex::Symbol { .. }) = w {
                        adjacent_symbols += 1;
                        return false;
                    }

                    true
                });

                if adjacent_symbols > 0 {
                    part1 += num;
                }
            }
            &Vertex::Symbol { c: '*', x: _, y: _ } => {
                let mut adjacent_numbers = 0;
                let mut res = 1;

                graph.for_each_neighbor(i, |w| {
                    if let Some(Vertex::Number { num, .. }) = w {
                        res *= num;
                        adjacent_numbers += 1;
                        return adjacent_numbers < 2;
                    }

                    true
                });

                if adjacent_numbers == 2 {
                    part2 += res;
                }
            }
            _ => {}
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

struct Graph {
    vertices: Vec<Vertex>,
    pos_to_vertex: HashMap<(i32, i32), usize, ahash::RandomState>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            pos_to_vertex: HashMap::default(),
        }
    }

    fn insert(&mut self, mut v: Vertex) {
        let next_index = self.vertices.len();

        match &mut v {
            Vertex::Symbol { c: _, x, y } => {
                self.pos_to_vertex.insert((*x, *y), next_index);
            }
            Vertex::Number {
                num: _,
                y,
                x,
                xlen: len,
            } => {
                for dx in 0..*len {
                    self.pos_to_vertex.insert((*x + dx, *y), next_index);
                }
            }
        }

        self.vertices.push(v);
    }

    /// Execute function for every neighbor of vertex with index i **exactly once**.
    /// The function should return false if it wishes to stop early (and true if it whishes to continue)
    fn for_each_neighbor<F>(&self, i: usize, mut f: F)
    where
        F: FnMut(Option<&Vertex>) -> bool,
    {
        let v = &self.vertices[i];
        let (y, x, xlen) = match v {
            Vertex::Number {
                num: _,
                y,
                x,
                xlen: len,
            } => (*y, *x, *len),
            Vertex::Symbol { c: _, x, y } => (*y, *x, 1),
        };

        // Vec is faster than hashset (there are usually barely any entries in here)
        // so run with that
        let mut visited = Vec::new();

        for dy in -1..=1 {
            for dx in -1..=xlen {
                // In bounds, do not check the digits themselves
                if dy == 0 && 0 <= dx && dx < xlen {
                    continue;
                }
                if let Some(idx) = self.pos_to_vertex.get(&(x + dx, y + dy)).copied() {
                    if visited.contains(&idx) {
                        continue;
                    }
                    visited.push(idx);

                    let res = f(self.vertices.get(idx));
                    if !res {
                        return;
                    }
                }
            }

            // We can only find the same vertex twice if we stay on the same y level
            visited.clear();
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Vertex {
    Symbol { c: char, x: i32, y: i32 },
    Number { num: i64, y: i32, x: i32, xlen: i32 },
}
