#![allow(dead_code)]

const INPUT: &str = include_str!("../input/day12.txt");
pub fn solve() -> crate::Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in INPUT.lines() {
        let record = Record::parse(line);
        part1 += record.count_solutions();
        part2 += record.unfold().count_solutions();
        println!("Line done");
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}

#[derive(Debug)]
struct Record {
    elems: Vec<SpringState>,
    signature: Vec<usize>,
}

impl Record {
    fn unfold(&self) -> Self {
        let mut elems = Vec::new();
        let mut signature = Vec::new();
        for i in 0..5 {
            if i != 0 {
                elems.push(SpringState::Unknown);
            }
            elems.extend_from_slice(&self.elems);
            signature.extend_from_slice(&self.signature);
        }

        Self { elems, signature }
    }

    fn parse(s: &str) -> Self {
        let (elems, spec) = s.split_once(' ').unwrap_or((s, ""));
        let elems = elems
            .chars()
            .map(|c| SpringState::try_from(c).unwrap())
            .collect::<Vec<_>>();
        let spec = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            elems,
            signature: spec,
        }
    }

    fn count_solutions(&self) -> usize {
        fn recurse(parser: Parser<'_>, elems: &[SpringState]) -> usize {
            // Do something like LOOK AHEAD, and fail fast if there is just no point at all
            // (i.e. number of placeholders cant possibly be enough or something)
            
            match elems.first() {
                None => return parser.success() as usize,
                Some(SpringState::Operational) => {
                    if let Some(p) = parser.read_operational() {
                        recurse(p, &elems[1..])
                    } else {
                        0
                    }
                }
                Some(SpringState::Damaged) => {
                    if let Some(p) = parser.read_damaged() {
                        recurse(p, &elems[1..])
                    } else {
                        0
                    }
                }
                Some(SpringState::Unknown) => {
                    let mut ret = 0;
                    let mut success = false;

                    if let Some(p) = parser.read_operational() {
                        ret += recurse(p, &elems[1..]);
                        success = true;
                    }
                    if let Some(p) = parser.read_damaged() {
                        ret += recurse(p, &elems[1..]);
                        success = true;
                    }
                    debug_assert!(success, "One of the two should always be possible...");

                    ret
                }
            }
        }

        let parser = Parser {
            signature: &self.signature,
            debt: None,
        };

        recurse(parser, &self.elems)
    }
}

#[derive(Clone, Debug)]
struct Parser<'a> {
    signature: &'a [usize],
    debt: Option<usize>,
}

impl Parser<'_> {
    fn success(&self) -> bool {
        self.signature.is_empty() && (self.debt == None || self.debt == Some(0))
    }

    fn read_operational(&self) -> Option<Self> {
        let ret = match self.debt {
            Some(0) => Self {
                debt: None,
                signature: self.signature,
            },
            Some(_) => return None,
            None => Self {
                debt: None,
                signature: self.signature,
            },
        };

        Some(ret)
    }

    fn read_damaged(&self) -> Option<Self> {
        let ret = match self.debt {
            Some(0) => return None,
            Some(debt) => Self {
                debt: Some(debt - 1),
                signature: self.signature,
            },
            None if self.signature.is_empty() => return None,
            None => {
                let next_debt = self.signature[0] - 1;
                Self {
                    debt: Some(next_debt),
                    signature: &self.signature[1..],
                }
            }
        };

        Some(ret)
    }
}

#[derive(Copy, Clone, PartialEq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl std::fmt::Debug for SpringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl TryFrom<char> for SpringState {
    type Error = crate::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => return Err(crate::Error::InvalidInput),
        };

        Ok(res)
    }
}

#[test]
fn example() {
    let lines = [
        "?.### 1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1"
    ];
    let should_be1 = [
        1, 4, 1, 1, 4, 10
    ];
    let should_be2 = [
        1, 16384, 1, 16, 2500, 506250
    ];

    for i in 0..lines.len() {
        assert_eq!(Record::parse(lines[i]).count_solutions(), should_be1[i]);
    }

    for i in 0..lines.len() {
        let rec = Record::parse(lines[i]).unfold();
        println!("{:?}", rec);
        assert_eq!(rec.count_solutions(), should_be2[i]);
    }
}
