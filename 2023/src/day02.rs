use crate::Error;

const INPUT: &str = include_str!("../input/day02.txt");

pub fn solve() -> crate::Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;

    for game in INPUT.lines() {
        let (game_id, rest) = game
            .split_once(": ")
            .ok_or("Line is prefixed by something ended with a ':'")?;

        let game_id = game_id
            .strip_prefix("Game ")
            .ok_or("Every line starts with prefix 'Game'")?
            .parse::<i64>()?;

        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);

        for set in rest.split("; ") {
            for cube in set.split(", ") {
                let (num, cube) = cube.split_once(' ').ok_or(Error::InvalidInput)?;
                let num = num.parse::<i64>()?;
                match cube {
                    "red" => max_r = max_r.max(num),
                    "green" => max_g = max_g.max(num),
                    "blue" => max_b = max_b.max(num),
                    _ => return Err(Error::InvalidInput),
                };
            }
        }

        if max_r <= 12 && max_g <= 13 && max_b <= 14 {
            part1 += game_id;
        }

        part2 += max_r * max_g * max_b;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
