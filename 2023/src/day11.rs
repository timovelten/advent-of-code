#[cfg(test)]
use indoc::indoc;

const INPUT: &str = include_str!("../input/day11.txt");

pub fn solve() -> crate::Result<()> {
    let grid = Grid::parse(INPUT).unwrap();
    let (part1, part2) = solve_impl(&grid, 1_000_000);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}

fn solve_impl(grid: &Grid, multiplier2: u64) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..grid.galaxies.len() {
        for j in (i + 1)..grid.galaxies.len() {
            let from = grid.galaxies[i];
            let to = grid.galaxies[j];
            part1 += grid.taxicab_distance(from, to, 2);
            part2 += grid.taxicab_distance(from, to,  multiplier2);
        }
    }

    (part1, part2)
}


#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

struct Grid {
    galaxies: Vec<Point>,
    big_rows: Vec<i32>,
    big_cols: Vec<i32>,
}

impl Grid {
    fn parse(s: &str) -> crate::Result<Self> {
        let mut lines = s.trim().lines().peekable();

        let width = lines
            .peek()
            .map(|first_line| first_line.chars().count() as i32)
            .unwrap_or(0);

        let mut big_rows = Vec::default();
        let mut big_cols = Vec::default();
        let mut galaxies = Vec::default();

        // Initially, all columns are big
        big_cols.extend(0..width);

        for (y, line) in s.trim().lines().enumerate() {
            let y = y as i32;

            let mut w = 0;
            let mut big_row = true;
            for (x, c) in line.char_indices() {
                let x = x as i32;
                if c == '#' {
                    galaxies.push(Point { x, y });
                    big_row = false;

                    // Remote this column, its clear not empty
                    if let Some(j) = big_cols.iter().position(|elem| *elem == x) {
                        big_cols.remove(j);
                    }
                }

                w += 1;
            }

            if width != 0 && width != w {
                return Err("inconsistent line width".into());
            }

            if big_row {
                big_rows.push(y);
            }
        }
        
        Ok(Self {
            galaxies,
            big_rows,
            big_cols,
        })
    }

    fn taxicab_distance(&self, from: Point, to: Point, big_galaxy_cost: u64) -> u64 {
        // To calculate this acurately, we have to determine how many of the "large galaxies"
        // lie in the x and y range
        let (min_x, max_x) = (from.x.min(to.x), from.x.max(to.x));
        let (min_y, max_y) = (from.y.min(to.y), from.y.max(to.y));

        let mut large_x = 0;
        for &gx in &self.big_cols {
            if min_x <= gx && gx <= max_x {
                large_x += 1;
            }
        }

        let mut large_y = 0;
        for &gy in &self.big_rows {
            if min_y <= gy && gy <= max_y {
                large_y += 1;
            }
        }

        // The large galaxies cost more...
        let x_cost = (max_x - min_x - large_x) as u64 + (large_x as u64) * big_galaxy_cost;
        let y_cost = (max_y - min_y - large_y) as u64 + large_y as u64 * big_galaxy_cost;
        x_cost + y_cost
    }    
}

#[test]
fn example() {
    let example = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    let grid = Grid::parse(example).unwrap();
    assert_eq!(grid.taxicab_distance(Point { x: 3, y: 0 }, Point { x: 7, y: 8}, 2), 15);
    assert_eq!(grid.taxicab_distance(Point { x: 0, y: 9 }, Point { x: 4, y: 9}, 2), 5);
    assert_eq!(grid.taxicab_distance(Point { x: 0, y: 2 }, Point { x: 9, y: 6}, 2), 17);

    let (part1, part2) = solve_impl(&grid, 100);
    assert_eq!(part1, 374);
    assert_eq!(part2, 8410);
}
