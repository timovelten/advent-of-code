use crate::Error;

const INPUT: &str = include_str!("../input/day06.txt");

pub fn solve() -> crate::Result<()> {
    let mut lines = INPUT.lines();
    let line1 = lines.next().ok_or(Error::InvalidInput)?;
    let line2 = lines.next().ok_or(Error::InvalidInput)?;

    let times = parse_nos(line1)?;
    let distances = parse_nos(line2)?;
    if times.len() != distances.len() {
        return Err(Error::InvalidInput);
    }

    let mut part1 = 1;
    for i in 0..times.len() {
        part1 *= count_ways_to_win(times[i], distances[i]);
    }

    let big_time = concatenate(times.iter().copied());
    let big_distance = concatenate(distances.iter().copied());
    let part2 = count_ways_to_win(big_time, big_distance);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

/// One should really do this with string concatenation. But hey, its also
/// possible without allocating.
fn concatenate(iter: impl Iterator<Item=u64>) -> u64 {
    let mut ret = 0;
    for item in iter {
        let shift = item.ilog10() + 1;
        ret = ret * 10u64.pow(shift) + item;
    }
    ret
}

fn parse_nos(input: &str) -> crate::Result<Vec<u64>> {
    let res = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(res)
}

fn count_ways_to_win(duration: u64, record: u64) -> u64 {
    // If t is the time we take to "prepare", then the distance we travel will be
    //     (duration - t) * t
    // That is, the task is to solve the quadratic inequliaty (duration - t) * t > record

    // This can be done using the quadratic formula, essentially.
    // The solutions of t^2 - duration * t + record = 0 are
    // x = 0.5 * (duration +- sqrt(duration^2 - 4 record))
    // The challenge now is to find the largest and smallest INTEGER solutions.
    // Esentially we have to determine the floor and ceil of these values, while
    // avoiding exact solutions.
    let disc = duration * duration - 4 * record;

    // Determine the floor of the lower solution
    let mut lower = (duration - isqrt(disc)).div_euclid(2);
    let mut upper = (duration + isqrt(disc)).div_euclid(2);

    if (duration - lower) * lower <= record {
        lower += 1;
    }

    if (duration - upper) * upper <= record {
        upper -= 1;
    }

    upper - lower + 1

    // Note: We have used the mathematical fact that
    // floor((a + sqrt(d))/b) = floor((a + isqrt(d)) /b)
    // where isqrt(d) is the floor of the sqrt.
}

fn isqrt(y: u64) -> u64 {
    if y == 0 {
        return 0;
    }

    let (mut l, mut r) = (1, y - 1);
    while l < r {
        let mid = l + (r - l + 1) / 2;
        let go_left = if let Some(mid_squared) = mid.checked_mul(mid) {
            mid_squared <= y
        } else {
            false
        };

        if go_left {
            l = mid;
        } else {
            r = mid - 1;
        }
    }

    l
}

#[test]
fn test_isqrt() {
    assert_eq!(isqrt(1212374), 1101);
    assert_eq!(isqrt(12345 * 12345), 12345);
}

#[test]
fn test_example() {
    assert_eq!(count_ways_to_win(7, 9), 4);
    assert_eq!(count_ways_to_win(15, 40), 8);
    assert_eq!(count_ways_to_win(30, 200), 9);
}
