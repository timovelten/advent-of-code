const INPUT: &str = include_str!("../input/day01.txt");

pub fn solve() -> crate::Result<()> {
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    for line in INPUT.lines() {
        // Part 1
        let mut real_digits = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|x| x as u64);

        // The evaluation order matters here!
        if let Some(first_digit) = real_digits.next() {
            let last_digit = real_digits.last().unwrap_or(first_digit);
            part1 += first_digit * 10 + last_digit;
        }

        // Part 2
        let first_digit = first_digit(line);
        let last_digit = last_digit(line);
        if let (Some(a), Some(b)) = (first_digit, last_digit) {
            part2 += a as u64 * 10 + b as u64;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

fn parse_digit_suffix(s: &str) -> Option<u8> {
    if s.len() == 0 {
        return None;
    }

    const DIGITS: [(u8, &'static str); 9] = [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];

    let last_char = s.chars().last().unwrap();
    if let Some(digit) = last_char.to_digit(10) {
        return Some(digit as u8);
    }

    for (digit, digit_str) in DIGITS {
        if s.ends_with(digit_str) {
            return Some(digit);
        }
    }

    None
}

fn first_digit(s: &str) -> Option<u8> {
    for (i, _) in s.char_indices() {
        if let Some(digit) = parse_digit_suffix(&s[..=i]) {
            return Some(digit);
        }
    }

    None
}

fn last_digit(s: &str) -> Option<u8> {
    for (i, _) in s.char_indices().rev() {
        if let Some(digit) = parse_digit_suffix(&s[..=i]) {
            return Some(digit);
        }
    }

    None
}
