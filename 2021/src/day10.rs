const INPUT: &str = include_str!("../input/day10");

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines();

    let mut prob1 = 0;
    let mut autocomplete_scores = Vec::new();

    let mut opened: Vec<char> = Vec::new();

    for line in lines {
        opened.clear();

        let mut first_illegal_character = None;

        for c in line.chars() {
            if matches!(c, '(' | '[' | '{' | '<') {
                opened.push(c);
            } else if let Some(x) = opened.pop() {
                let expected = match x {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => unreachable!()
                };

                if c != expected {
                    first_illegal_character = Some(c);
                    break;
                }
            }
        }

        if let Some(x) = first_illegal_character {
            // Corrupted line
            prob1 += match x {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            };
        } else {
            // Incomplete line
            let mut score: u64 = 0;

            for &c in opened.iter().rev() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!()
                };
            }

            autocomplete_scores.push(score);
        }
    }

    println!("Problem 1: {}", prob1);

    autocomplete_scores.sort();
    println!("Problem 2: {}", autocomplete_scores[autocomplete_scores.len() / 2]);

    Ok(())
}