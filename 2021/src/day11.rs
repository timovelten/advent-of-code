const INPUT: &str = include_str!("../input/day11");

fn checked_add(a: usize, b: isize) -> Option<usize> {
    if b >= 0 {
        a.checked_add(b as usize)
    } else {
        a.checked_sub(-b as usize)
    }
}

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines();

    let mut grid = lines
        .map(|line| {
            line.chars()
                .map(|x| -> Option<u8> { Some(x.to_digit(10)? as u8) })
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(crate::Error::InvalidInput)?;

    let width = grid.first().ok_or(crate::Error::NoInput)?.len();

    let height = grid.len();

    pub fn increase(
        x: usize,
        y: usize,
        grid: &mut Vec<Vec<u8>>,
        width: usize,
        height: usize,
    ) -> u64 {
        grid[y][x] += 1;

        // Only flash once
        if grid[y][x] == 10 {
            let mut ret = 1;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if let (Some(nx), Some(ny)) = (checked_add(x, dx), checked_add(y, dy)) {
                        if nx >= width || ny >= height {
                            continue;
                        }

                        ret += increase(nx, ny, grid, width, height);
                    }
                }
            }

            ret
        } else {
            0
        }
    }

    let mut prob1 = 0;
    let mut prob2 = None;

    for step in 0.. {
        let mut flashed_this_step = 0;

        for y in 0..height {
            for x in 0..width {
                flashed_this_step += increase(x, y, &mut grid, width, height);
            }
        }

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] > 9 {
                    grid[y][x] = 0;
                }
            }
        }

        if step < 100 {
            prob1 += flashed_this_step;
        }

        if prob2 == None && flashed_this_step == (width * height) as u64 {
            prob2 = Some(step + 1);
        }

        if step >= 100 && prob2.is_some() {
            break;
        }
    }

    println!("Problem 1: {}", prob1);
    println!("Problem 2: {}", prob2.unwrap());

    Ok(())
}
