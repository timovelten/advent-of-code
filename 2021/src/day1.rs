const INPUT: &str = include_str!("../input/day1");

pub fn solve() -> Result<(), crate::Error> {
    let nrs = INPUT
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Part 1
    let mut prob1 = 0;

    for i in 1..nrs.len() {
        if nrs[i] > nrs[i - 1] {
            prob1 += 1;
        }
    }

    println!("Problem 1: {}", prob1);

    // Part 2
    let mut prob2 = 0;

    for i in 3..nrs.len() {
        let (a, b, c, d) = (nrs[i - 3], nrs[i - 2], nrs[i - 1], nrs[i]);
        if a + b + c < b + c + d {
            prob2 += 1;
        }
    }

    println!("Problem 2: {}", prob2);

    Ok(())
}

pub fn solve_without_allocating() -> Result<(), crate::Error> {
    let nrs = INPUT.lines().map(|x| x.parse::<u64>().unwrap());

    let mut prob1 = 0;
    let mut prev = None;

    let mut prob2 = 0;

    let mut a = None;
    let mut b = None;
    let mut prev_sum = None;

    for next in nrs {
        // Part 1
        if let Some(prev) = prev {
            if next > prev {
                prob1 += 1;
            }
        }

        prev = Some(next);

        // Part 2
        let c = next;
        let mut sum = None;
        if let (Some(a), Some(b)) = (a, b) {
            sum = Some(a + b + c);
        }

        if let (Some(prev_sum), Some(sum)) = (prev_sum, sum) {
            if prev_sum < sum {
                prob2 += 1;
            }
        }

        a = b;
        b = Some(c);

        prev_sum = sum;
    }

    println!("Problem 1: {}", prob1);
    println!("Problem 1: {}", prob2);

    Ok(())
}
