use std::time::{Instant, Duration};
use ahash::HashMap;
#[cfg(test)]
use indoc::indoc;

const INPUT: &str = include_str!("../input/day14.txt");

pub fn solve() -> crate::Result<()> {
    let mut grid1 = Grid::parse(INPUT);
    let mut grid2 = grid1.clone();
    
    grid1.tilt_to_north();
    let part1 = grid1.determine_load();

    grid2.cycle(1_000_000_000);
    let part2 = grid2.determine_load();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}


#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    elems: Vec<Option<Rock>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn set(&mut self, x: i32, y: i32, elem: Option<Rock>) {
        self.elems[(x as usize) + (y as usize) * (self.width as usize)] = elem;
    }

    fn get(&self, x: i32, y: i32) -> Option<Rock> {
        self.elems[(x as usize) + (y as usize) * (self.width as usize)]
    }
    
    fn fall_helper(&mut self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        let rock = self.get(x, y);
        if rock != Some(Rock::Movable) {
            return false;
        }

        let (mut nx, mut ny) = (x, y);
        loop {
            let (nxdx, nydy) = (nx + dx, ny + dy);
            let in_bounds = 0 <= nxdx && nxdx < self.width && 0 <= nydy && nydy < self.height;
            if !in_bounds || self.get(nxdx, nydy).is_some() {
                break;
            }
            (nx, ny) = (nxdx, nydy);
        }

        // The order here matters...
        self.set(x, y, None);
        self.set(nx, ny, rock);

        (x, y) != (nx, ny)
    }

    fn tilt_to_north(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.fall_helper(x, y, 0, -1);
            }
        }
    }

    fn tilt_to_south(&mut self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                self.fall_helper(x, y, 0, 1);
            }
        }
    }

    fn tilt_to_west(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.fall_helper(x, y, -1, 0);
            }
        }
    }

    fn tilt_to_east(&mut self) {
        for x in (0..self.width).rev() {
            for y in 0..self.height {
                self.fall_helper(x, y, 1, 0);
            }
        }
    }

    fn cycle_once(&mut self) {
        self.tilt_to_north();
        self.tilt_to_west();
        self.tilt_to_south();
        self.tilt_to_east();
    }    

    /// If this returns Some((i, j)) then the state self is in after i and j
    /// cycles is the same (and this is also the current value of self, when this returns).
    /// If this returns None, then self has been cycled exactly n times, and all intermediate
    /// states were distinct.
    fn cycle_helper(&mut self, n: Option<usize>) -> Option<(usize, usize)> {
        let mut states = HashMap::default();
        states.insert(self.clone(), 0);

        let mut cycle_time = Duration::ZERO;
        let mut insert_time = Duration::ZERO;

        for i in 1..n.unwrap_or(usize::MAX) {
            let before = Instant::now();
            self.cycle_once();
            cycle_time += before.elapsed();

            let before = Instant::now();
            let res = states.insert(self.clone(), i);
            insert_time += before.elapsed();

            if let Some(old) = res {
                return Some((old, i));
            }
        }

        None
    }

    fn cycle(&mut self, n: usize) {
        if let Some((i, j)) = self.cycle_helper(Some(n)) {
            // Extrapolate from here..i
            let rem = (n - j) % (j - i);
            for _ in 0..rem {
                self.cycle_once();
            }
        } else {
            // If cycle_helper returns none it has run exactly n cycles, so this is what we are looking for
            return;
        }
    }

    fn determine_load(&self) -> u64 {
        let mut ret = 0;

        for y in 0..self.height {
            let moveable_rocks = (0..self.width)
                .filter_map(|x| {
                    self.get(x, y)
                        .filter(|rock| *rock == Rock::Movable)
                })
                .count() as u64;
            ret += (self.height as u64 - y as u64) * moveable_rocks;
        }

        ret
    }

    fn parse(s: &str) -> Self {
        let s = s.trim();

        let width = s.lines().next()
            .map(|line| line.chars().count())
            .unwrap_or(0);
        let height = s.lines().count();

        let elems = vec![None; width * height];
        let mut ret = Self {
            elems,
            width: width as i32,
            height: height as i32
        };
        for (y, line) in s.lines().enumerate() {
            let line = line.trim();
            for (x, c) in line.char_indices() {
                let rock = match c {
                    '#' => Rock::Solid,
                    'O' => Rock::Movable,
                    _ => continue,
                };

                ret.set(x as i32, y as i32, Some(rock));
            }
        }

        ret
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.get(x, y) {
                    Some(Rock::Solid) => '#',
                    Some(Rock::Movable) => 'O',
                    None => '.',
                };

                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Rock {
    Solid,
    Movable,
}

#[test]
fn example() {
    let example = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....      
    "};

    let mut grid1 = Grid::parse(example);
    let mut grid2 = Grid::parse(example);

    println!("{}", grid1);

    grid1.tilt_to_north();
    println!("{}", grid1);
    assert_eq!(grid1.determine_load(), 136);

    // Part 2
    grid2.cycle(1_000_000_000);
    assert_eq!(grid2.determine_load(), 64);
}
