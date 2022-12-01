use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day8");
const N: usize = 7;
const CHARS: [char; N] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
const DIGIT_PATTERNS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

type Permutation = [u8; N];
type Pattern = [u8; N];

pub fn parse_pattern(s: &str) -> Pattern {
    let mut ret = [0; N];

    for i in 0..N {
        if s.contains(CHARS[i]) {
            ret[i] = 1;
        }
    }

    ret
}

pub fn permute(pattern: &Pattern, pi: &Permutation) -> Pattern {
    let mut ret = [0; N];

    for i in 0..N {
        ret[pi[i] as usize] = pattern[i]
    }

    ret
}

pub fn solve() -> crate::Result<()> {
    let observations = INPUT
        .lines()
        .map(|line| {
            let mut split = line.split(" | ").map(|part| {
                part.split_whitespace()
                    .map(|pat| parse_pattern(pat))
                    .collect::<Vec<_>>()
            });

            (split.next().unwrap(), split.next().unwrap())
        })
        .collect::<Vec<_>>();

    // Determine a permutation pi such that for every input binary pattern
    // (x6, ..., x0) it holds that (x_pi^-1(6), ..., x^pi^-1(0)) is a valid pattern
    fn make_pi<F: Fn(&Pattern) -> bool>(
        mut pi: Permutation,
        i: usize,
        used: &mut HashSet<u8>,
        inp: &Vec<Pattern>,
        valid_pattern: &F,
    ) -> Option<Permutation> {
        if i == N {
            if used.len() < N {
                return None;
            }

            for pattern in inp {
                let permuted = permute(pattern, &pi);

                if !valid_pattern(&permuted) {
                    return None;
                }
            }

            Some(pi)
        } else {
            for j in 0..(N as u8) {
                if used.contains(&j) {
                    continue;
                }

                used.insert(j);
                pi[i] = j;

                if let Some(pi) = make_pi(pi, i + 1, used, inp, valid_pattern) {
                    return Some(pi);
                }

                used.remove(&j);
            }

            None
        }
    }

    let digit_patterns: HashMap<Pattern, u8> = DIGIT_PATTERNS
        .iter()
        .enumerate()
        .map(|(i, s)| (parse_pattern(s), i as u8))
        .collect();

    let mut prob1 = 0;
    let mut prob2 = 0;

    for (inp, outp) in observations {
        let pi = make_pi([0; N], 0, &mut Default::default(), &inp, &|p: &Pattern| {
            digit_patterns.contains_key(p)
        })
        .ok_or(crate::Error::InvalidInput)?;

        let mut tmp: u64 = 0;
        for w in outp {
            tmp *= 10;
            tmp += digit_patterns[&permute(&w, &pi)] as u64;

            match w.iter().sum() {
                2 | 3 | 4 | 7 => prob1 += 1,
                _ => {}
            }
        }

        prob2 += tmp;
    }

    println!("Problem 1: {}", prob1);
    println!("Problem 2: {}", prob2);

    Ok(())
}
