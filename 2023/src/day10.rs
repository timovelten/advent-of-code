use std::collections::{vec_deque, VecDeque};

use ahash::{HashMap, HashSet};
use indoc::indoc;

const INPUT: &str = include_str!("../input/day10.txt");

// IDEA FOR TOMOROW: Go in "half steps" and check if there is a "half spot" between pipes. It is annoying, but will probably
// work. 

pub fn solve() -> crate::Result<()> {
    let grid = Grid::parse(INPUT).unwrap();
    let (part1, part2) = solve_impl(&grid);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");


    Ok(())
}

fn solve_impl(grid: &Grid) -> (usize, usize) {
    let loop_in_grid = grid.find_loop().unwrap();
    let part1 = loop_in_grid.len() / 2;

    let outside_tiles = grid.outside_tiles(
        loop_in_grid.iter().copied().collect(),
        (-1, -1),
        (-1, -1),
        (grid.width + 2, grid.height + 2),
    );

    for y in -1..(grid.height + 1) {
        for x in -1..(grid.width + 1) {
            if outside_tiles.contains(&(x, y)) {
                print!("x");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // Observe: all-tiles = outside-tiles + inside-tiles + loop-tiles
    let part2 = (grid.width + 2) as usize * (grid.height + 2) as usize 
            - loop_in_grid.len()
            - outside_tiles.len();

    (part1, part2)
}

struct Grid {
    width: i32,
    height: i32,
    tiles: HashMap<(i32, i32), Tile>,
    start: (i32, i32),
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
                tiles.insert((x, y), tile);

                if tile == Tile::Start {
                    if start.is_some() {
                        return Err("multiple start points".into());
                    }
                    start = Some((x, y));
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

    fn blow_up(&self) -> Self {
        let mut tiles: HashMap<(i32, i32), Tile> = HashMap::default();
        let width = self.width * 2;
        let height = self.height * 2;
        let mut tiles = HashMap::default();

        for y in 0..height {
            for x in 0..width {
                if let Some(&tile) = self.tiles.get(&(x, y)) {
                    // a b
                    // c d
                    let (a, b) = ((2 * x, 2 * y), (2 * x + 1, 2 * y));
                    let (c, d) = ((2 * x, 2 * y + 1), (2 * x + 1, 2 * y + 1));
                    
                    tiles.insert(a, tile);
                    match tile {
                        Tile::Vertical => {
                            tiles.insert(c, Tile::Vertical);
                        },
                        Tile::Horizontal => {
                            tiles.insert(b, Tile::Horizontal);
                        },
                        Tile::SouthWest => {
                            tiles.insert(c, Tile::Vertical);
                        },
                        Tile::SouthEast => {
                            tiles.insert(c, Tile::Vertical);
                            tiles.insert(b, Tile::Horizontal);
                        },
                        Tile::NorthEast => {
                            tiles.insert(b, Tile::Horizontal);
                        }
                        Tile::NorthWest => {},
                        _ => {}
                    }
                }
            }
        }


        Self {
            width, height, tiles, start: (self.start.0 * 2, self.start.1 * 2)
        }
    }

    fn find_loop(&self) -> Option<Vec<(i32, i32)>> {
        fn recurse(
            grid: &Grid,
            pos: (i32, i32),
            prev: (i32, i32),
            out: &mut Vec<(i32, i32)>,
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
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let mut out = vec![self.start];
            let res = recurse(
                self,
                (self.start.0 + dx, self.start.1 + dy),
                self.start,
                &mut out,
            );

            if res.is_some() {
                return Some(out);
            }
        }

        None
    }

    fn outside_tiles(
        &self,
        boundary: HashSet<(i32, i32)>,
        pos: (i32, i32),
        (min_x, min_y): (i32, i32),
        (width, height): (i32, i32),
    ) -> HashSet<(i32, i32)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::default();
        queue.push_front(pos);
        visited.insert(pos);
        
        while let Some((x, y)) = queue.pop_back() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = (x + dx, y + dy);
                let inbounds = min_x <= nx 
                    && nx < min_x + width 
                    && min_y <= ny 
                    && ny < min_y + height;
                let newpos = (nx, ny);
                if visited.contains(&newpos) || boundary.contains(&newpos) || !inbounds {
                    continue;
                }

                queue.push_front(newpos);
                visited.insert(newpos);
            }
        }

        visited
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

    fn adjacent_at(&self, (x, y): (i32, i32)) -> Option<[(i32, i32); 2]> {
        let res = match self {
            Self::Vertical => [(x, y + 1), (x, y - 1)],
            Self::Horizontal => [(x + 1, y), (x - 1, y)],
            Self::NorthEast => [(x, y - 1), (x + 1, y)],
            Self::NorthWest => [(x, y - 1), (x - 1, y)],
            Self::SouthWest => [(x - 1, y), (x, y + 1)],
            Self::SouthEast => [(x + 1, y), (x, y + 1)],
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

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;
        for x in 0..self.width {
            write!(f, "{}", x % 10)?;
        }
        write!(f, "\n")?;

        for y in 0..self.height {
            write!(f, "{}", y % 10)?;
            for x in 0..self.width {
                let tile = self.tiles.get(&(x, y)).copied().unwrap_or(Tile::Ground);
                write!(f, "{tile}")?;
            }
            write!(f, "\n")?;
        }

        Ok(())
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
    // let grid = grid.blow_up();
    println!("{}", grid);
    panic!();
}
