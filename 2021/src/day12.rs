use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day12");

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Vertex {
    // This is quite a greedy optimization, but the number of vertices seems to be way smaller
    // than 256
    id: u8,
    visit_once: bool,
}

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines();

    let mut vertices: HashMap<String, Vertex> = Default::default();
    let mut adjacent: HashMap<Vertex, HashSet<Vertex>> = Default::default();

    let mut get_vertex = |s: &str| {
        if let Some(v) = vertices.get(s) {
            *v
        } else {
            let s_is_lower = s.chars().all(|c| c.is_lowercase());

            let v = Vertex {
                id: vertices.len() as u8,
                visit_once: s_is_lower,
            };

            vertices.insert(s.to_owned(), v);

            v
        }
    };

    for line in lines {
        let mut split = line.trim().split("-");

        let v = get_vertex(split.next().ok_or(crate::Error::InvalidInput)?);
        let w = get_vertex(split.next().ok_or(crate::Error::InvalidInput)?);

        adjacent.entry(v).or_default().insert(w);
        adjacent.entry(w).or_default().insert(v);
    }

    // f(start, end, ...) counts the number of paths from start to end
    // if visit twice is set, it only counts those paths that visit the passed vertex *exactly*
    // twice.
    fn f(
        start: Vertex,
        end: Vertex,
        visit_twice: Option<Vertex>,
        path: &mut HashSet<Vertex>,
        adjacent: &HashMap<Vertex, HashSet<Vertex>>,
    ) -> u64 {
        let inserted = path.insert(start);

        let mut ret = 0;

        if start == end {
            if visit_twice.is_none() {
                ret = 1;
            } else {
                // return 0
            }
        } else {
            for &v in &adjacent[&start] {
                let mut new_visit_twice = visit_twice;

                if v.visit_once && path.contains(&v) {
                    if Some(v) == visit_twice {
                        // This vertex is now visited for the second time, but we allow it
                        new_visit_twice = None;
                    } else {
                        // Otherwise, visiting this vertex twice is not allowed
                        continue;
                    }
                }

                ret += f(v, end, new_visit_twice, path, adjacent);
            }
        }

        if inserted {
            path.remove(&start);
        }

        ret
    }

    let start = vertices["start"];
    let end = vertices["end"];

    let mut temp_path = Default::default();

    let prob1 = f(
        start,
        end,
        None,
        &mut temp_path,
        &adjacent,
    );

    println!("Problem 1: {}", prob1);

    let mut prob2 = prob1;

    for &v in vertices.values() {
        if v.visit_once && v != start && v != end {
            prob2 += f(start, end, Some(v), &mut temp_path, &adjacent);
        }
    }

    println!("Problem 2: {}", prob2);

    Ok(())
}
