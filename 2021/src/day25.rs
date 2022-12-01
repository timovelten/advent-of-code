use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

const INPUT: &str = include_str!("../input/day25");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    East,
    South,
}

#[derive(Clone)]
struct Grid {
    spaces: Vec<Option<Direction>>,
    width: usize,
    height: usize,
}

impl Index<(usize, usize)> for Grid {
    type Output = Option<Direction>;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.spaces[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.spaces[x + y * self.width]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let letter = match self[(x, y)] {
                    Some(Direction::East) => '>',
                    Some(Direction::South) => 'v',
                    None => '.',
                };

                write!(f, "{}", letter)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn step_east(&mut self) -> usize {
        let mut changed = 0;

        let first_col: Vec<_> = (0..self.height)
            .map(|y| self[(0, y)])
            .collect();

        for y in 0..self.height {
            let mut skip_next = false;
            for x in 0..self.width {
                if skip_next {
                    skip_next = false;
                    continue;
                }

                if self[(x, y)] != Some(Direction::East) {
                    continue;
                }
                
                if x + 1 < self.width && self[(x + 1, y)].is_none()  {
                    self[(x + 1, y)] = self[(x, y)].take();
                    skip_next = true;
                    changed += 1;
                } else if x + 1 == self.width && first_col[y].is_none() {
                    self[(0, y)] = self[(x, y)].take();
                    changed += 1;
                }
            }
        }

        changed
    }

    fn step_south(&mut self) -> usize {
        // Similar to step_east
        let mut changed = 0;

        let first_row: Vec<_> = (0..self.width)
            .map(|x| self[(x, 0)])
            .collect();

        for x in 0..self.width {
            let mut skip_next = false;
            for y in 0..self.height {
                if skip_next {
                    skip_next = false;
                    continue;
                }

                if self[(x, y)] != Some(Direction::South) {
                    continue;
                }
                
                if y + 1 < self.height && self[(x, y + 1)].is_none()  {
                    self[(x, y + 1)] = self[(x, y)].take();
                    skip_next = true;
                    changed += 1;
                } else if y + 1 == self.height && first_row[x].is_none() {
                    self[(x, 0)] = self[(x, y)].take();
                    changed += 1;
                }
            }
        }

        changed
    }

    fn step(&mut self) -> usize {
        self.step_east() + self.step_south()
    }
}

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines();

    let mut spaces = Vec::new();
    let mut width = 0;

    for line in lines {
        if width == 0 {
            width = line.len();
        }

        for c in line.chars() {
            let dir = match c {
                '.' => None,
                '>' => Some(Direction::East),
                'v' => Some(Direction::South),
                _ => return Err(crate::Error::InvalidInput),
            };

            spaces.push(dir);
        }
    }

    let mut grid = Grid {
        height: spaces.len() / width,
        width,
        spaces,
    };

    for i in 1.. {
        let changed = grid.step();
        if changed == 0 {
            println!("Problem 1: {}", i);
            break;
        }
    }

    Ok(())
}
