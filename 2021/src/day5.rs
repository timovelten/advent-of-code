use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day5");

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines();

    let mut hit_count_1: HashMap<(i32, i32), usize> = HashMap::new();
    let mut hit_count_2: HashMap<(i32, i32), usize> = HashMap::new();

    for line in lines {
        let mut nrs = line.split(" -> ")
            .flat_map(|p| p.split(","))
            .map(|x| x.parse::<i32>());

        let x1 = nrs.next().ok_or(crate::Error::NoInput)??;
        let y1 = nrs.next().ok_or(crate::Error::NoInput)??;
        let x2 = nrs.next().ok_or(crate::Error::NoInput)??;
        let y2 = nrs.next().ok_or(crate::Error::NoInput)??;

        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let straight = dx == 0 || dy == 0;

        let (mut x, mut y) = (x1, y1);

        loop {
            if straight {
                let e = hit_count_1.entry((x, y)).or_insert(0);
                *e += 1;
            }

            let e = hit_count_2.entry((x, y)).or_insert(0);
            *e += 1;

            if x == x2 && y == y2 {
                break;
            }

            x += dx;
            y += dy;
        }
    }

    let prob1 = hit_count_1.values().filter(|&&v| v >= 2).count();
    let prob2 = hit_count_2.values().filter(|&&v| v >= 2).count();

    println!("Problem 1: {}", prob1);
    println!("Problem 2: {}", prob2);

    Ok(())
}