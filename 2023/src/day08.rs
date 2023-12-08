use ahash::HashMap;

const INPUT: &str = include_str!("../input/day08.txt");

pub fn solve() -> crate::Result<()> {
    let mut lines = INPUT.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == 'L')
        .collect::<Vec<_>>();

    let mut map = HashMap::default();
    let mut start_points = Vec::new();
    
    for line in INPUT.lines().skip(2) {
        let (lhs, rhs) = line.split_once(" = ").unwrap();
        let (left, right) = rhs.split_once(", ").unwrap();
        let left = left.strip_prefix('(').unwrap();
        let right = right.strip_suffix(')').unwrap();

        map.insert(lhs.to_owned(), (left.to_owned(), right.to_owned()));

        if lhs.ends_with('A') {
            start_points.push(lhs.to_owned());
        }
    }

    let tmp = analyze(&map, &instructions, "AAA", |x| x == "ZZZ");
    let part1 = tmp
        .values()
        .next()
        .unwrap();

    println!("Part 1: {part1}");

    let n = start_points.len();
    let steps_to_endpoint = start_points
        .into_iter()
        .map(|s| analyze(&map, &instructions, &s, |x| x.ends_with('Z')))
        .map(|data| data.into_values().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part2 = u64::MAX;
    // Brute force: We each start point, we pick and endpoints (there may multiple*)
    // *in fact, it seems like there is only ever one :/, so this solution is overcomplicated
    let mut picked = vec![0; n];
    loop {
        let mut steps_for_choice = 1;
        for i in 0..n {
            steps_for_choice = lcm(steps_for_choice, steps_to_endpoint[i][picked[i]]);
        }
        if steps_for_choice < part2 {
            part2 = steps_for_choice;
        }

        let mut carry = 1;
        for i in 0..picked.len() {
            picked[i] += carry;
            if picked[i] >= steps_to_endpoint[i].len() {
                picked[i] = 0;
                carry = 1;
            }
        }

        if carry != 0 {
            break
        }
    }

    println!("Part 2: {part2}");
    Ok(())
}

fn analyze<F: Fn(&str) -> bool>(
    map: &HashMap<String, (String, String)>,
    instructions: &Vec<bool>,
    start: &str,
    end: F,
) -> HashMap<String, u64> {
    let mut res = HashMap::default();

    let mut steps = 0u64;
    let mut pos = start;
    loop {
        if end(pos) {
            if res.contains_key(pos) {
                break;
            }
            res.insert(pos.to_owned(), steps);
        }

        let inst = instructions[steps as usize % instructions.len()];
        let (l, r) = &map[pos];
        if inst {
            pos = l.as_str();
        } else {
            pos = r.as_str();
        }
        steps += 1;
    }

    res
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        let r = a.rem_euclid(b);
        if r == 0 {
            return b;
        }
        (a, b) = (b, r);
    }
}

const fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
