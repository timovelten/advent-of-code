use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day6");

pub fn solve() -> crate::Result<()> {
    let fish = INPUT.lines().next()
        .ok_or(crate::Error::NoInput)?
        .split(",")
        .map(|x| x.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?;

    let (prob1, prob2) = solve_memoized_recursion_table(&fish)?;

    // solve_dp_table takes around 0.6 ms (662900 ns)
    // solve_momoized_recursion_table takes around 0.6 ms  (675900 ns)
    // solve_dp_hashmap takes around 1.0ms (1009100 ns)
    // solve_momoized_recursion takes around 1.1 ms  (1142700 ns)

    // Conclusion: recursion is acceptable, using a hashmap instead of an array
    // is around two times slower (which is also acceptable, especially in the context of
    // competitive programming)

    println!("Problem 1: {}", prob1);
    println!("Problem 2: {}", prob2);

    Ok(())
}

fn solve_dp_table(fish: &Vec<u8>) -> crate::Result<(u64, u64)> {
    const DAYS: usize = 256;
    let mut dp = vec![0u64; (DAYS + 1) * 9];

    // dp[x + n * 9] will the number of fish the fish x turns into after n days
    for x in 0..=8 {
        dp[x] = 1;
    }

    // Now, fill each row of the dp table
    for i in 1..=DAYS {
        dp[0 + i * 9] = dp[6 + (i - 1) * 9] + dp[8 + (i - 1) * 9];

        for x in 1..=8 {
            dp[x + i * 9] = dp[(x - 1) + (i - 1) * 9];
        }
    }

    let prob1: u64 = fish.iter().map(|&x| dp[x as usize + 80 * 9]).sum();
    let prob2: u64 = fish.iter().map(|&x| dp[x as usize + 256 * 9]).sum();

    Ok((prob1, prob2))
}

fn solve_memoized_recursion_table(fish: &Vec<u8>) -> crate::Result<(u64, u64)> {
    const DAYS: u16 = 256;

    let mut cache = vec![0u64; (DAYS as usize + 1) * 9];

    fn f(x: u8, n: u16, cache: &mut Vec<u64>) -> u64 {
        let idx = (x as usize) + (n as usize) * 9;
        if cache[idx] != 0 {
            return cache[idx];
        }

        let ret = if n == 0 {
            1
        } else if x == 0 {
            f(6, n - 1, cache) + f(8, n - 1, cache)
        } else {
            f(x - 1, n - 1, cache)
        };

        cache[idx] = ret;
        ret
    }

    let prob1: u64 = fish.iter().map(|&x| f(x, 80, &mut cache)).sum();
    let prob2: u64 = fish.iter().map(|&x| f(x, 256, &mut cache)).sum();

    Ok((prob1, prob2))
}

fn solve_dp_hashmap(fish: &Vec<u8>) -> crate::Result<(u64, u64)> {
    const DAYS: u16 = 256;

    let mut dp: HashMap<(u8, u16), u64> = Default::default();

    for x in 0..=8 {
        dp.insert((x, 0), 1);
    }

    for i in 1..=DAYS {
        dp.insert((0, i), dp[&(6, (i - 1))] + dp[&(8, (i - 1))]);

        for x in 1..=8 {
            dp.insert((x, i), dp[&(x - 1, i - 1)]);
        }
    }

    let prob1: u64 = fish.iter().map(|&x| dp[&(x, 80)]).sum();
    let prob2: u64 = fish.iter().map(|&x| dp[&(x, 256)]).sum();

    Ok((prob1, prob2))
}

fn solve_memoized_recursion(fish: &Vec<u8>) -> crate::Result<(u64, u64)> {
    const DAYS: u16 = 256;

    let mut cache: HashMap<(u8, u16), u64> = Default::default();
    fn f(x: u8, n: u16, cache: &mut HashMap<(u8, u16), u64>) -> u64 {
        if let Some(ret) = cache.get(&(x, n)) {
            return *ret;
        }

        let ret = if n == 0 {
            1
        } else if x == 0 {
            f(6, n - 1, cache) + f(8, n - 1, cache)
        } else {
            f(x - 1, n - 1, cache)
        };

        cache.insert((x, n), ret);
        ret
    }

    let prob1: u64 = fish.iter().map(|&x| f(x, 80, &mut cache)).sum();
    let prob2: u64 = fish.iter().map(|&x| f(x, 256, &mut cache)).sum();

    Ok((prob1, prob2))
}