use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day14");

pub fn solve() -> crate::Result<()> {
    let mut lines = INPUT.lines();

    let input = lines
        .next()
        .ok_or(crate::Error::NoInput)?
        .chars()
        .collect::<Vec<char>>();

    // Skip the following empty line
    let _ = lines.next();

    let rules = lines
        .map(|line| -> Option<((char, char), char)> {
            let mut s = line.split(" -> ");

            let mut lhs = s.next()?.chars();
            let (lhs1, lhs2) = (lhs.next()?, lhs.next()?);

            let rhs = s.next()?.chars().next()?;

            Some(((lhs1, lhs2), rhs))
        })
        .collect::<Option<HashMap<(char, char), char>>>()
        .ok_or(crate::Error::InvalidInput)?;

    let all_chars: HashSet<char> = input
        .iter()
        .copied()
        .chain(rules.values().copied())
        .collect();

    // f((a, b), c, steps, ...) is the number of c's that are *added* to (a, b) after expanding it
    // steps number of steps.
    fn f(
        (a, b): (char, char),
        c: char,
        steps: usize,
        rules: &HashMap<(char, char), char>,
        cache: &mut HashMap<(char, char, char, usize), u64>,
    ) -> u64 {
        if let Some(&ret) = cache.get(&(a, b, c, steps)) {
            return ret;
        }

        let mut ret = 0;

        if steps > 0 {
            if let Some(&x) = rules.get(&(a, b)) {
                if x == c {
                    ret += 1;
                }

                // The pattern will turn into a x b so now we analyze a x and x b
                ret +=
                    f((a, x), c, steps - 1, rules, cache) + f((x, b), c, steps - 1, rules, cache);
            }
        }

        cache.insert((a, b, c, steps), ret);

        ret
    }

    let mut cache = Default::default();

    let mut base_qty: HashMap<char, u64> = Default::default();
    for &c in &input {
        *base_qty.entry(c).or_insert(0) += 1;
    }

    let mut qtys1 = base_qty.clone();
    let mut qtys2 = base_qty;

    for c in all_chars {
        for i in 0..input.len() - 1 {
            let (a, b) = (input[i], input[i + 1]);

            *qtys2.entry(c).or_insert(0) += f((a, b), c, 40, &rules, &mut cache);
            *qtys1.entry(c).or_insert(0) += f((a, b), c, 10, &rules, &mut cache);
        }
    }

    println!(
        "Problem 1: {}",
        qtys1.values().max().unwrap() - qtys1.values().min().unwrap()
    );
    println!(
        "Problem 2: {}",
        qtys2.values().max().unwrap() - qtys2.values().min().unwrap()
    );

    Ok(())
}
