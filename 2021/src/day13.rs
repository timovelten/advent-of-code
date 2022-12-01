use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day13");

type Point = (u32, u32);

fn fold_x(axis_x: u32, grid: &mut HashSet<Point>, width: &mut u32, height: u32) {
    if axis_x >= *width {
        return;
    }

    for x in 0..axis_x {
        let other_x = 2 * axis_x - x;

        if other_x >= *width {
            continue;
        }

        for y in 0..height {
            if grid.contains(&(other_x, y)) {
                grid.insert((x, y));
            }
        }
    }

    *width = axis_x;
}

fn fold_y(axis_y: u32, grid: &mut HashSet<Point>, width: u32, height: &mut u32) {
    if axis_y >= *height {
        return;
    }

    for y in 0..axis_y {
        let other_y = 2 * axis_y - y;

        if other_y >= *height {
            continue;
        }

        for x in 0..width {
            if grid.contains(&(x, other_y)) {
                grid.insert((x, y));
            }
        }
    }

    *height = axis_y;
}

fn print_grid(grid: &HashSet<Point>, width: u32, height: u32) {
    for y in 0..height {
        for x in 0..width {
            if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

pub fn solve() -> crate::Result<()> {
    let mut lines = INPUT.lines();

    let mut grid: HashSet<Point> = Default::default();

    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let mut split = line.split(',').map(|v| v.parse::<u32>());

        let x = split.next().ok_or(crate::Error::NoInput)??;
        let y = split.next().ok_or(crate::Error::NoInput)??;

        grid.insert((x, y));
        max_x = std::cmp::max(x, max_x);
        max_y = std::cmp::max(y, max_y);
    }

    let mut width = max_x + 1;
    let mut height = max_y + 1;

    let mut prob1 = None;

    for line in lines {
        let mut split = line.split('=');
        let s = split.next();
        let axis = split
            .next()
            .ok_or(crate::Error::InvalidInput)?
            .parse::<u32>()?;

        match s {
            Some("fold along x") => fold_x(axis, &mut grid, &mut width, height),
            Some("fold along y") => fold_y(axis, &mut grid, width, &mut height),
            _ => {}
        }

        if prob1.is_none() {
            let entries = grid
                .iter()
                .copied()
                .filter(|&(x, y)| x <= width && y <= height)
                .count();

            prob1 = Some(entries);
        }
    }

    println!("Problem 1: {}", prob1.unwrap());

    println!("Problem 2:");
    print_grid(&grid, width, height);

    Ok(())
}