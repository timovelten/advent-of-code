use ahash::HashMap;

#[cfg(test)]
use indoc::indoc;

const INPUT: &str = include_str!("../input/day14.txt");

pub fn solve() -> crate::Result<()> {
    let mut grid = Grid::parse(INPUT);
    grid.tilt();

    let part1 = grid.determine_load();

    println!("Part 1: {part1}");
    Ok(())
}

struct Grid {
    elems: HashMap<(i32, i32), Rock>,
    width: i32,
    height: i32,
}

impl Grid {
    fn tilt(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let rock = match self.elems.get(&(x, y)) {
                    Some(Rock::Movable) => Rock::Movable,
                    Some(Rock::Solid) => continue,
                    None => continue,
                };

                let mut new_y = y;
                while self.elems.get(&(x, new_y - 1)).is_none() && new_y >= 1 {
                    new_y -= 1;
                }
                self.elems.remove(&(x, y));
                self.elems.insert((x, new_y), rock);
            }
        }
    }

    fn determine_load(&self) -> u64 {
        let mut ret = 0;

        for y in 0..self.height {
            let moveable_rocks = (0..self.width)
                .filter_map(|x| {
                    self.elems
                        .get(&(x, y))
                        .filter(|rock| **rock == Rock::Movable)
                })
                .count() as u64;
            ret += (self.height as u64 - y as u64) * moveable_rocks;
        }

        ret
    }

    fn parse(s: &str) -> Self {
        let s = s.trim();
        let mut elems = HashMap::default();

        let mut height = 0;
        let mut width = 0;
        for (y, line) in s.lines().enumerate() {
            let line = line.trim();
            for (x, c) in line.char_indices() {
                if y == 0 {
                    width += 1;
                }

                let rock = match c {
                    '#' => Rock::Solid,
                    'O' => Rock::Movable,
                    _ => continue,
                };

                elems.insert((x as i32, y as i32), rock);
            }

            height += 1;
        }

        Self {
            elems,
            width,
            height,
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.elems.get(&(x, y)) {
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

    let mut grid = Grid::parse(example);
    grid.tilt();
    assert_eq!(grid.determine_load(), 136);
}
