#[cfg(test)]
use indoc::indoc;

const INPUT: &str = include_str!("../input/day07.txt");

pub fn solve() -> crate::Result<()> {
    let (part1, part2) = solve_impl(INPUT);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}

fn solve_impl(input: &str) -> (i64, i64) {
    let mut hands = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let (hand, bid) = (parse_hand(hand.trim()), bid.trim().parse::<i64>().unwrap());
        hands.push((hand, bid));
    }

    hands.sort_by_cached_key(|(a, _)| (determine_hand_type(a), a.clone()));

    let part1 = hands.iter()
        .copied()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as i64 * bid)
        .sum();

    hands.sort_by_cached_key(|(a, _)| {
        let best_type = determine_best_possible_type(a);
        let mut tie_breaker = [None; 5];
        for i in 0..5 {
            if a[i] != Card::J {
                tie_breaker[i] = Some(a[i]);
            }
        }

        (best_type, tie_breaker)
    });
    
    let part2 = hands.iter()
        .copied()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as i64 * bid)
        .sum();
    
    (part1, part2)
}

fn parse_hand(s: &str) -> [Card; 5] {
    let mut ret = [Card::No2; 5];
    let mut len = 0;
    for (i, c) in s.chars().enumerate() {
        ret[i] = Card::from_char(c);
        len += 1;
    }
    assert_eq!(len, 5);

    ret
}

fn determine_hand_type<T: Ord + Clone>(hand: &[T; 5]) -> HandType {
    let mut hand = hand.clone();
    hand.sort_unstable();
    let mut longest_run = 0;
    let mut distinct = 1;
    let mut run = 1;
    for i in 1..5 {
        if hand[i - 1] == hand[i] {
            run += 1;
        } else {
            longest_run = longest_run.max(run);
            run = 1;
            distinct += 1;
        }
    }
    longest_run = longest_run.max(run);

    if distinct == 1 {
        HandType::FiveOfAKind
    } else if distinct == 2 {
        // Can either be full house or four of a kind
        if longest_run == 4 {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        }
    } else if distinct == 2 {
        HandType::FourOfAKind
    } else if distinct == 3 {
        // Can either be three of a kind or two pair
        if longest_run == 3 {
            HandType::ThreeOfAkind
        } else {
            HandType::TwoPair
        }
    } else if distinct == 4 {
        HandType::OnePair
    } else if distinct == 5 {
        HandType::HighCard
    } else {
        unreachable!()
    }
}

fn determine_best_possible_type(hand: &[Card; 5]) -> HandType  {
    // Observation: The actual cards do not matter for the type. Therefore, to simplify,
    // we convert the given cards into internal cards that are just 5 integers (each of which
    // lies in the range -1, 0, 1, 2, 3)
    let mut internal_hand = [-1i8; 5];
    let mut card_numbers = Vec::new();
    for (i, card) in hand.iter().enumerate() {
        if *card == Card::J {
            internal_hand[i] = -1;
        } else if let Some(j) = card_numbers.iter().copied().position(|x| x == card) {
            internal_hand[i] = j as i8;
        } else {
            internal_hand[i] = card_numbers.len() as i8;
            card_numbers.push(card);
        }
    }

    fn recurse(hand: &[i8; 5]) -> HandType {
        let placeholder_position = match hand.iter().copied().position(|x| x == -1) {
            Some(i) => i,
            None => return determine_hand_type(hand)
        };

        let mut new_hand = hand.clone();
        let mut result = None;
        for new in 0..=3 {
            new_hand[placeholder_position] = new;
            let tmp = recurse(&new_hand);
            result = result.max(Some(tmp));
        }

        result.unwrap()
        
    }

    recurse(&internal_hand)    
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAkind,
    FullHouse, 
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    No2,
    No3,
    No4,
    No5,
    No6,
    No7,
    No8,
    No9,
    T,
    J,
    Q,
    K,
    A
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::No2,
            '3' => Self::No3,
            '4' => Self::No4,
            '5' => Self::No5,
            '6' => Self::No6,
            '7' => Self::No7,
            '8' => Self::No8,
            '9' => Self::No9,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("Cant interpret {c} as card")
        }
    }
}

#[test]
fn hand_types() {
    assert_eq!(determine_hand_type(&parse_hand("32T3K")), HandType::OnePair);
    assert_eq!(determine_hand_type(&parse_hand("KTJJT")), HandType::TwoPair);
    assert_eq!(determine_hand_type(&parse_hand("KK677")), HandType::TwoPair);
    assert_eq!(determine_hand_type(&parse_hand("T55J5")), HandType::ThreeOfAkind);
    assert_eq!(determine_hand_type(&parse_hand("QQQJA")), HandType::ThreeOfAkind);
}

#[test]
fn best_possible_hand_types() {
    assert_eq!(
        determine_best_possible_type(&parse_hand("T55J5")),
        HandType::FourOfAKind
    );
    assert_eq!(
        determine_best_possible_type(&parse_hand("T55J5")),
        HandType::FourOfAKind
    );
    assert_eq!(
        determine_best_possible_type(&parse_hand("T55J5")),
        HandType::FourOfAKind
    );
}

#[test]
fn example() {
    let example = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483    
    "};
    let (part1, part2) = solve_impl(example);
    assert_eq!(part1, 6440);
    assert_eq!(part2, 5905);
}
