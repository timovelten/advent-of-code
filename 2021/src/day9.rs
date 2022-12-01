use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day9");

pub fn adjacent_locations(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(move |&(dx, dy)| {
            let (nx, ny) = ((x as isize) + dx, (y as isize) + dy);

            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
}

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines();

    let mut grid: Vec<u8> = Vec::new();
    let mut width = 0;

    for line in lines {
        let parsed = line
            .trim()
            .chars()
            .map(|c| char::to_digit(c, 10).unwrap() as u8);

        grid.extend(parsed);

        if width == 0 {
            width = grid.len();
        }
    }

    let height = grid.len() / width;

    let mut prob1: u64 = 0;

    let mut basin_sizes: Vec<usize> = Default::default();

    for x in 0..width {
        for y in 0..height {
            let mut lowpoint = true;
            let val = grid[x + y * width];

            for (nx, ny) in adjacent_locations(x, y, width, height) {
                if grid[nx as usize + ny as usize * width] <= val {
                    lowpoint = false;
                    break;
                }
            }

            if !lowpoint {
                continue;
            }

            prob1 += val as u64 + 1;

            let mut basin: HashSet<(usize, usize)> = Default::default();
            let mut to_be_checked: Vec<(usize, usize)> = vec![(x, y)];

            while let Some((x, y)) = to_be_checked.pop() {
                basin.insert((x, y));

                let val = grid[x + y * width];

                for (nx, ny) in adjacent_locations(x, y, width, height) {
                    let nval = grid[nx + ny * width];

                    if nval < 9 && nval > val && !basin.contains(&(nx, ny)) {
                        to_be_checked.push((nx, ny));
                    }
                }
            }

            basin_sizes.push(basin.len());
        }
    }

    println!("Problem 1: {}", prob1);

    basin_sizes.sort();
    let prob2: u64 = basin_sizes
        .iter()
        .rev()
        .take(3)
        .map(|x| *x as u64)
        .product();

    println!("Problem 2: {}", prob2);

    Ok(())
}
