use std::collections::VecDeque;
use ahash::{HashMap, HashSet};

#[cfg(test)]
use indoc::indoc;

const INPUT: &str = include_str!("../input/day10.txt");

// IDEA FOR TOMOROW: Go in "half steps" and check if there is a "half spot" between pipes. It is annoying, but will probably
// work. 

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct HalfPoint {
    x: i32,
    y: i32
}

impl std::fmt::Debug for HalfPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x as f64 / 2.0, self.y as f64 / 2.0)
    }
}

impl HalfPoint {
    fn from_whole(x: i32, y: i32) -> Self {
        Self { x: 2 * x, y: 2 * y }
    }
    
    fn translate(self, x: i32, y: i32) -> Self {
        Self { x: self.x + 2 * x, y: self.y + 2 * y }
    }

    fn whole_neighbors(self) -> [HalfPoint; 4] {
        [
            self.translate(1, 0),
            self.translate(-1, 0),
            self.translate(0, 1),
            self.translate(0, -1)
        ]
    }

    fn fractional_neighbors(self) -> [HalfPoint; 4] {
        [
            self.translate_fractional(1, 0),
            self.translate_fractional(-1, 0),
            self.translate_fractional(0, 1),
            self.translate_fractional(0, -1),
        ]
    }

    fn translate_fractional(self, x: i32, y: i32) -> Self {
        Self { x: self.x + x, y: self.y + y }
    }

    fn is_whole(self) -> bool {
        self.x % 2 == 0 && self.y % 2 == 0
    }

    fn midpoint(self, other: HalfPoint) -> Self {
        Self { x: (self.x + other.x) / 2, y: (self.y + other.y) / 2 }
    }
}

pub fn solve() -> crate::Result<()> {
    let grid = Grid::parse(INPUT).unwrap();
    let (part1, part2) = solve_impl(&grid);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");


    Ok(())
}

fn solve_impl(grid: &Grid) -> (usize, usize) {
    let boundary = grid.find_loop().unwrap();
    let part1 = boundary.len() / 2;

    let filled_in_boundary = Grid::fill_in_circle(&boundary);

    let outside_tiles = grid.outside_tiles(
        filled_in_boundary.iter().copied().collect(),
        HalfPoint { x: -1, y: -1 },
        grid.width,
        grid.height
    );

    let part2 = grid.width as usize * grid.height as usize
        - outside_tiles
        - boundary.len();

    (part1, part2)
}

struct Grid {
    width: i32,
    height: i32,
    tiles: HashMap<HalfPoint, Tile>,
    start: HalfPoint,
}

impl Grid {
    fn parse(s: &str) -> crate::Result<Self> {
        let mut tiles = HashMap::default();
        let mut width = None;
        let mut height = 0;
        let mut start = None;

        for (y, line) in s.trim().lines().enumerate() {
            let y = y as i32;

            let mut w = 0;
            for (x, c) in line.char_indices() {
                let x = x as i32;

                w += 1;
                let tile = Tile::from(c)?;
                tiles.insert(HalfPoint::from_whole(x, y), tile);

                if tile == Tile::Start {
                    if start.is_some() {
                        return Err("multiple start points".into());
                    }
                    start = Some(HalfPoint::from_whole(x, y));
                }
            }

            if width.is_some_and(|x| x != w) {
                return Err("inconsistent row with".into());
            }
            width = Some(w);
            height += 1;
        }

        Ok(Self {
            height,
            width: width.unwrap_or(0),
            tiles,
            start: start.ok_or("no start")?,
        })
    }

    fn find_loop(&self) -> Option<Vec<HalfPoint>> {
        fn recurse(
            grid: &Grid,
            pos: HalfPoint,
            prev: HalfPoint,
            out: &mut Vec<HalfPoint>,
        ) -> Option<()> {
            out.push(pos);

            // Find where to go next. Each tile has two tiles it can go to. We just have to elminate the one
            // we came from, to find out where we are going.
            let tile = grid.tiles.get(&pos)?;
            let adjacent = tile.adjacent_at(pos)?;
            let j = adjacent.iter().position(|point| *point == prev)?;
            let next_pos = adjacent[(j + 1) % 2];

            if next_pos == grid.start {
                Some(())
            } else {
                recurse(grid, next_pos, pos, out)
            }
        }

        // We have to try up to three directions potentially...But try all four
        // just to be safe
        for neighbor in self.start.whole_neighbors() {
            let mut out = vec![self.start];
            let res = recurse(
                self,
                neighbor,
                self.start,
                &mut out
            );

            if res.is_some() {
                return Some(out);
            }
        }

        None
    }

    fn fill_in_circle(circle: &Vec<HalfPoint>) -> Vec<HalfPoint> {
        // On a regular path with n vertices, there are n - 1 spaces to fill.
        let mut ret = Vec::new();
        let n = circle.len();
        for i in 0..n {
            let current = circle[i % n];
            let next = circle[(i + 1) % n];
            let midpoint = current.midpoint(next);

            ret.push(current);
            ret.push(midpoint);
        }

        ret
    }

    fn outside_tiles(
        &self,
        boundary: HashSet<HalfPoint>,
        pos: HalfPoint,
        width: i32,
        height: i32
    ) -> usize {        
        let mut count = 0;
        
        let mut queue = VecDeque::new();
        let mut visited = HashSet::default();
        queue.push_front(pos);
        visited.insert(pos);
        
        while let Some(point) = queue.pop_back() {
            if point.is_whole() {
                count += 1;
            }
            
            for neighbor in point.fractional_neighbors() {
                let in_bounds_x = pos.x <= neighbor.x && neighbor.x - pos.x < 2 * width + 1;
                let in_bounds_y = pos.y <= neighbor.y && neighbor.y - pos.y < 2 * height + 1;
                if visited.contains(&neighbor) || boundary.contains(&neighbor) {
                    continue;
                }
                if !in_bounds_x || !in_bounds_y {
                    continue;
                }

                queue.push_front(neighbor);
                visited.insert(neighbor);
            }
        }

        count
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
}

impl Tile {
    fn from(c: char) -> crate::Result<Self> {
        let res = match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::Start,
            '.' | 'I' | 'O' => Self::Ground,
            _ => return Err(format!("cant convert {c} to").into()),
        };

        Ok(res)
    }

    fn adjacent_at(&self, pos: HalfPoint) -> Option<[HalfPoint; 2]> {
        let res = match self {
            Self::Vertical => [pos.translate(0, 1), pos.translate(0, -1)],
            Self::Horizontal => [pos.translate(1, 0), pos.translate(-1, 0)],
            Self::NorthEast => [pos.translate(0, -1), pos.translate(1, 0)],
            Self::NorthWest => [pos.translate(0, -1), pos.translate(-1, 0)],
            Self::SouthWest => [pos.translate(-1, 0), pos.translate(0, 1)],
            Self::SouthEast => [pos.translate(1, 0), pos.translate(0, 1)],
            _ => return None,
        };

        Some(res)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => write!(f, "│"),
            Self::Horizontal => write!(f, "─"),
            Self::NorthEast => write!(f, "└"),
            Self::NorthWest => write!(f, "┘"),
            Self::SouthWest => write!(f, "┐"),
            Self::SouthEast => write!(f, "┌"),
            Self::Ground => write!(f, "·"),
            Self::Start => write!(f, "S"),
        }
    }
}

#[test]
fn example1() {
    let example = indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF            
    "};

    let grid = Grid::parse(example).unwrap();
    let (part1, _) = solve_impl(&grid);
    assert_eq!(part1, 4);
}

#[test]
fn example2() {
    let example = indoc! {"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...            
    "};
    let grid = Grid::parse(example).unwrap();
    let (part1, _) = solve_impl(&grid);
    assert_eq!(part1, 8);
}

#[test]
fn example3() {
    let example = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...     
    "};
    let grid = Grid::parse(example).unwrap();
    let (_, part2) = solve_impl(&grid);
    assert_eq!(part2, 8);
}

#[test]
fn example4() {
    let example = indoc! {"
         FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJIF7FJ-
        L---JF-JLJIIIIFJLJJ7
        |F|F-JF---7IIIL7L|7|
        |FFJF7L7F-JF7IIL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L       
    "};
    let grid = Grid::parse(example).unwrap();
    let (_, part2) = solve_impl(&grid);
    assert_eq!(part2, 10);
}

#[test]
fn example5() {
    let example = indoc!{"
        ..........
        .S------7.
        .|F----7|.
        .||OOOO||.
        .||OOOO||.
        .|L-7F-J|.
        .|II||II|.
        .L--JL--J.
        ..........            
    "};
    let grid = Grid::parse(example).unwrap();
    let (_, part2) = solve_impl(&grid);
    assert_eq!(part2, 4);
}
