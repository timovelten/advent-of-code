const INPUT: &str = include_str!("../input/day7");

pub fn solve() -> crate::Result<()> {
    let line = INPUT
        .lines()
        .next()
        .ok_or(crate::Error::NoInput)?;

    let crabs = line
        .split(',')
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    // Initialze a few values
    let max = crabs.iter().copied().max().ok_or(crate::Error::NoInput)?;

    let prob1 = minimize(&crabs, max, |x: i32, i: i32| (x - i).abs());

    let prob2 = minimize(&crabs, max, |x: i32, i: i32| {
        let n = (x - i).abs();

        // Note: this should really be 0.5 * n * (n + 1) but we have factored this out to
        // to save on multiplication cost
        n * (n + 1)
    }) / 2;

    println!("Problem 1: {}", prob1);
    println!("Problem 2: {}", prob2);

    Ok(())
}

fn minimize<F: Fn(i32, i32) -> i32>(crabs: &Vec<i32>, max: i32, cost: F) -> i32 {
    let mut best = i32::MAX;

    // Just try every value and abort fast if the value is worse than our current 'best'
    for i in 0..=max {
        let mut current = 0;

        for c in crabs.chunks(8) {
            current += c.iter().map(|&x| cost(x, i)).sum::<i32>();

            if current >= best {
                break;
            }
        }

        if current < best {
            best = current;
        }
    }

    best
}
