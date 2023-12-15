#[cfg(test)]
use indoc::indoc;

const INPUT: &str = include_str!("../input/day13.txt");

pub fn solve() -> crate::Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;
    for block in INPUT.trim().split("\n\n") {
        let grid = Grid::parse(block);
        let grid_t = grid.transpose();

        part1 += grid_t
            .find_horizontal_reflection(false)
            .map(|x| x + 1)
            .unwrap_or(0);
        part1 += grid
            .find_horizontal_reflection(false)
            .map(|y| 100 * (y + 1))
            .unwrap_or(0);

        part2 += grid_t
            .find_horizontal_reflection(true)
            .map(|x| x + 1)
            .unwrap_or(0);
        part2 += grid
            .find_horizontal_reflection(true)
            .map(|y| 100 * (y + 1))
            .unwrap_or(0);
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Grid {
    rows: Vec<u64>,
    width: usize,
}

impl Grid {
    fn has(&self, x: usize, y: usize) -> bool {
        return (self.rows[y] >> x) & 1 == 1
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows.len() {
            for x in 0..self.width {
                if self.has(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn height(&self) -> usize {
        self.rows.len()
    }

    fn parse(s: &str) -> Self {
        let s = s.trim();
        let mut width = 0;

        let mut rows = Vec::default();
        for line in s.lines() {
            let mut row = 0;

            let mut w = 0;
            for (x, c) in line.char_indices() {
                if c == '#' {
                    row |= 1 << x
                }
                w += 1;
            }
            width = width.max(w);
            rows.push(row);
        }

        assert!(rows.len() <= 64 && width <= 64);

        Self { rows, width }
    }

    fn find_horizontal_reflection(&self, fix_smudge: bool) -> Option<usize> {
        // Find two adjacent rows that are alike, and then go from there..
        for y in 0..self.height() - 1 {
            if self.is_horizontal_reflection(y, fix_smudge) {
                return Some(y);
            }
        }

        None
    }

    fn is_horizontal_reflection(&self, y: usize, mut fix_smudge: bool) -> bool {
        let right = match self.height().checked_sub(y + 2) {
            Some(r) => r,
            None => return false
        };
        let h = y.min(right);

        for i in 0..=h {
            let top_row = self.rows[y - i];
            let bottom_row = self.rows[y + 1 + i];
            
            if top_row == bottom_row {
                continue;
            }

            if fix_smudge {
                if (top_row ^ bottom_row).count_ones() == 1 {
                    fix_smudge = false;
                    continue;
                }
            }

            return false;
        }

        // Usually, this should be true. This is here to make sure that if fix_smudge is set,
        // we do fix exactly (not less than) one smudge
        !fix_smudge
    }

    fn transpose(&self) -> Self {
        let width = self.height();

        let mut rows = Vec::default();
        for x in 0..self.width {
            let mut row = 0;
            for y in 0..self.height() {
                if self.has(x, y) {
                    row |= 1 << y;
                }
            }
            rows.push(row);
        }

        Self { rows, width }
    }
}

#[test]
fn example1() {
    let example = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
    "};

    let grid = Grid::parse(example);
    assert_eq!(grid.find_horizontal_reflection(false), None);
    assert_eq!(grid.transpose().find_horizontal_reflection(false), Some(4));
    assert_eq!(grid.find_horizontal_reflection(true), Some(2));
}

#[test]
fn example2() {
    let example = indoc! {"
       #...##..#
       #....#..#
       ..##..###
       #####.##.
       #####.##.
       ..##..###
       #....#..#
    "};

    let grid = Grid::parse(example);
    assert_eq!(grid.find_horizontal_reflection(false), Some(3));
    assert_eq!(grid.find_horizontal_reflection(true), Some(0));
}

#[test]
fn custom_example() {
    let example = indoc! {"
        ###.###..#....#..
        #....#.#.#.#####.
        #.#....#.....###.
        .###.##..######.#
        .....#....#.##.##
        .....#....#.##.##
        .###.##..######.#
        #.#....#.....###.
        ##...#.#.#.#####.
        ###.###..#....#..
        ....#...#.#......
        ..##.##.#.###.#..
        ..##.##.#.###.#..
    "};

    let grid = Grid::parse(example);
    assert_eq!(grid.find_horizontal_reflection(false), Some(11));
}
