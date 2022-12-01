const INPUT: &str = include_str!("../input/day18");

#[derive(Debug, Clone)]
struct SnailfishNumber {
    left: Number,
    right: Number,
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let left = Number::Other(self.into());
        let right = Number::Other(rhs.into());

        SnailfishNumber { left, right }
    }
}

impl SnailfishNumber {
    fn reduce(&mut self) {
        while self.reduce_once() {}
    }

    fn reduce_once(&mut self) -> bool {
        if self.explode_once() {
            return true;
        }

        self.split_once()
    }

    fn split_once(&mut self) -> bool {
        for x in [&mut self.left, &mut self.right] {
            let ret = match x {
                Number::Literal(l) if *l >= 10 => {
                    let div = *l / 2;

                    // Split
                    let left = Number::Literal(div);
                    let right = Number::Literal(*l - div);

                    let sn = SnailfishNumber { left, right };

                    *x = Number::Other(sn.into());

                    true
                }
                Number::Literal(_) => false,
                Number::Other(other) => other.split_once(),
            };

            if ret {
                return true;
            }
        }

        false
    }

    fn explode_once(&mut self) -> bool {
        let state = explode_sn(
            self,
            0,
            ReductionState {
                done_something: false,
                add_before: None,
                add_after: None,
            },
        );

        return state.done_something;

        struct ReductionState {
            done_something: bool,
            add_before: Option<u8>,
            add_after: Option<u8>,
        }

        fn add_to_first_literal(this: &mut Number, a: u8, go_left: bool) {
            match this {
                Number::Literal(x) => *x += a,
                Number::Other(other) => {
                    if go_left {
                        add_to_first_literal(&mut other.left, a, true);
                    } else {
                        add_to_first_literal(&mut other.right, a, false);
                    }
                }
            }
        }

        fn explode_sn(
            this: &mut SnailfishNumber,
            level: usize,
            state: ReductionState,
        ) -> ReductionState {
            let mut state = explode_number(&mut this.left, level, state);

            if let Some(a) = state.add_after.take() {
                add_to_first_literal(&mut this.right, a, true);
                state.add_after = None;
            }

            if state.done_something {
                return state;
            }

            // Go right
            let mut state = explode_number(&mut this.right, level, state);

            if let Some(a) = state.add_before.take() {
                add_to_first_literal(&mut this.left, a, false);
            }

            state
        }

        fn explode_number(
            this: &mut Number,
            level: usize,
            mut state: ReductionState,
        ) -> ReductionState {
            debug_assert!(!state.done_something);

            match this {
                Number::Other(x) if level >= 3 => {
                    // Explode!
                    let left = match x.left {
                        Number::Literal(x) => x,
                        _ => unreachable!("as per the problem description"),
                    };

                    let right = match x.right {
                        Number::Literal(x) => x,
                        _ => unreachable!("as per the problem description"),
                    };

                    *this = Number::Literal(0);

                    state.add_before = Some(left);
                    state.add_after = Some(right);
                    state.done_something = true;

                    state
                }
                Number::Other(x) => {
                    // Recurse
                    explode_sn(x, level + 1, state)
                }
                _ => state,
            }
        }
    }

    fn magnitude(&self) -> u64 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

#[derive(Debug, Clone)]
enum Number {
    Literal(u8),
    Other(Box<SnailfishNumber>),
}

impl Number {
    fn magnitude(&self) -> u64 {
        match self {
            Number::Literal(l) => *l as u64,
            Number::Other(x) => x.magnitude(),
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(x) => write!(f, "{}", x),
            Self::Other(n) => n.fmt(f),
        }
    }
}

fn parse_snailfish_number(s: &str) -> Option<(SnailfishNumber, &str)> {
    if !s.starts_with('[') {
        return None;
    }

    let (left, rem) = parse_number(&s[1..])?;

    if !rem.starts_with(',') {
        return None;
    }

    let (right, rem) = parse_number(&rem[1..])?;
    let sn = SnailfishNumber { left, right };

    if !rem.starts_with(']') {
        return None;
    }

    return Some((sn, &rem[1..]));
}

fn parse_number(s: &str) -> Option<(Number, &str)> {
    if let Some((n, rem)) = parse_snailfish_number(s) {
        Some((Number::Other(n.into()), rem))
    } else {
        let parsed_digit = s.chars().next()?.to_digit(10)? as u8;

        Some((Number::Literal(parsed_digit), &s[1..]))
    }
}

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines();
    let numbers = lines
        .map(|s| parse_snailfish_number(&s).map(|(n, _)| n))
        .collect::<Option<Vec<_>>>()
        .ok_or(crate::Error::InvalidInput)?;

    let mut res = numbers.first().ok_or(crate::Error::NoInput)?.clone();
    for x in &numbers[1..] {
        res = res + x.clone();
        res.reduce();
    }

    println!("Problem 1: {}", res.magnitude());

    let mut max_magnitude = 0;

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let mut res = numbers[i].clone() + numbers[j].clone();
            res.reduce();

            max_magnitude = std::cmp::max(res.magnitude(), max_magnitude);
        }
    }

    println!("Problem 2: {}", max_magnitude);

    Ok(())
}
