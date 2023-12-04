use ahash::{HashMap, HashMapExt};

const INPUT: &str = include_str!("../input/day04.txt");

pub fn solve() -> crate::Result<()> {
    // For every card, remember the number of winning cards it contained
    // we need this information for part2
    let mut xs = Vec::new();

    let mut part1 = 0;
    for line in INPUT.lines() {
        let line = line.strip_prefix("Card").ok_or("expected prefix 'Card'")?;
        let (i, rest) = line.split_once(':').unwrap();
        let card_no = i.trim().parse::<usize>()?;
        let (winning, have) = rest.split_once('|').unwrap();

        let winning_nos = parse_nos(winning);
        let have_nos = parse_nos(have).collect::<Vec<_>>();

        let num_winning_cards: usize = winning_nos.filter(|&i| have_nos.contains(&i)).count();

        let x = if let Some(k) = num_winning_cards.checked_sub(1) {
            1 << k
        } else {
            0
        };

        part1 += x;
        xs.resize(xs.len().max(card_no), 0);
        xs.insert(card_no, num_winning_cards);
    }

    let mut cache = HashMap::new();

    let mut part2 = 0;
    for i in 0..xs.len() {
        part2 += expand(i, &xs, &mut cache);
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

// Parse space separated numbers. Every number that fails to parse is silently ignored.
fn parse_nos(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.split_ascii_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
}

fn expand(i: usize, scores: &[usize], cache: &mut HashMap<usize, u64>) -> u64 {
    if let Some(&ret) = cache.get(&i) {
        return ret;
    }

    let num = scores[i];
    let mut ret = 1;
    for d in 1..=num {
        ret += expand(i + d, scores, cache);
    }

    cache.insert(i, ret);
    ret
}
