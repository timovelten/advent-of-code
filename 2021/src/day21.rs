use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day21");

fn solve_problem1(mut p: [u64; 2]) -> u64 {
    let mut score = [0, 0];
    let mut turn = 0;
    let mut dice = 1;

    let mut rounds = 0;

    loop {
        let sum = 3 * dice + 3;
        dice += 3;

        let new_space = (p[turn] + sum - 1) % 10 + 1;

        p[turn] = new_space;
        score[turn] += new_space;

        rounds += 1;

        if score[turn] >= 1000 {
            break;
        }

        turn = (turn + 1) % 2;
    }

    (rounds * 3) * (score[(turn + 1) % 2])
}

fn simulate_quantum(
    p: [u8; 2],
    score: [u8; 2],
    turn: usize,
    cache: &mut HashMap<([u8; 2], [u8; 2], u8), [u64; 2]>,
) -> [u64; 2] {
    if let Some(&ret) = cache.get(&(p, score, turn as u8)) {
        return ret;
    }

    let mut ret = [0, 0];
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                let sum = r1 + r2 + r3;

                let mut p = p.clone();
                let mut score = score.clone();

                let new_space = (p[turn] + sum - 1) % 10 + 1;

                p[turn] = new_space;
                score[turn] += new_space;

                if score[turn] >= 21 {
                    ret[turn] += 1;
                } else {
                    let s = simulate_quantum(p, score, (turn + 1) % 2, cache);

                    ret[0] += s[0];
                    ret[1] += s[1];
                }
            }
        }
    }

    cache.insert((p, score, turn as u8), ret);
    ret
}

pub fn solve() -> crate::Result<()> {
    let mut lines = INPUT.lines();

    let mut p = [0, 0];
    for i in 0..=1 {
        p[i] = lines
        .next()
        .ok_or(crate::Error::InvalidInput)?
        .split(": ")
        .nth(1)
        .ok_or(crate::Error::InvalidInput)?
        .parse::<u64>()?;
    }

    println!("Problem 1: {}", solve_problem1(p));

    println!(
        "Problem 2: {:?}",
        simulate_quantum([p[0] as u8, p[1] as u8], [0, 0], 0, &mut Default::default())
            .iter()
            .max()
            .unwrap()
    );

    Ok(())
}
