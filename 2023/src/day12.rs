use ahash::HashMap;

const INPUT: &str = include_str!("../input/day12.txt");

pub fn solve() -> crate::Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in INPUT.lines() {
        let record = Record::parse(line);
        part1 += record.count_solutions();
        part2 += record.unfold().count_solutions();
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}

#[derive(Debug)]
struct Record {
    data: Vec<Symbol>,
    signature: Vec<usize>,
}

impl Record {
    fn unfold(&self) -> Self {
        let mut data = Vec::new();
        let mut signature = Vec::new();
        for i in 0..5 {
            if i != 0 {
                data.push(Symbol::Unknown);
            }
            data.extend_from_slice(&self.data);
            signature.extend_from_slice(&self.signature);
        }

        Self { data, signature }
    }

    fn parse(s: &str) -> Self {
        let (elems, spec) = s.split_once(' ').unwrap_or((s, ""));
        let data = elems
            .chars()
            .map(|c| Symbol::try_from(c).unwrap())
            .collect::<Vec<_>>();
        let spec = spec
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            data,
            signature: spec,
        }
    }

    fn csol(&self, parser: Option<ParserState>, cache: &mut HashMap<ParserState, usize>) -> usize {
        let parser = match parser {
            Some(parser) => parser,
            None => return 0,
        };

        if parser.pos >= self.data.len() {
            return usize::from(parser.success(self));
        }

        if let Some(&ret) = cache.get(&parser) {
            return ret;
        }

        let ret = match self.data[parser.pos] {
            Symbol::Operational => self.csol(parser.read_operational(), cache),
            Symbol::Damaged => self.csol(parser.read_damaged(self), cache),
            Symbol::Unknown => {
                let op = self.csol(parser.read_operational(), cache);
                let damaged = self.csol(parser.read_damaged(self), cache);
                op + damaged
            }
        };

        cache.insert(parser, ret);

        ret
    }

    fn count_solutions(&self) -> usize {
        let parser = ParserState {
            pos: 0,
            sig_pos: 0,
            debt: None,
        };

        self.csol(Some(parser), &mut HashMap::default())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct ParserState {
    pos: usize,
    sig_pos: usize,
    debt: Option<usize>,
}

impl ParserState {
    fn success(&self, ctx: &Record) -> bool {
        self.sig_pos >= ctx.signature.len() && (self.debt.is_none() || self.debt == Some(0))
    }

    fn read_operational(&self) -> Option<Self> {
        let ret = match self.debt {
            Some(0) | None => Self {
                debt: None,
                sig_pos: self.sig_pos,
                pos: self.pos + 1,
            },
            Some(_) => return None,
        };

        Some(ret)
    }

    fn read_damaged(&self, ctx: &Record) -> Option<Self> {
        let ret = match self.debt {
            Some(0) => return None,
            Some(debt) => Self {
                debt: Some(debt - 1),
                sig_pos: self.sig_pos,
                pos: self.pos + 1,
            },
            None => {
                let next_debt = ctx.signature.get(self.sig_pos)?;
                Self {
                    debt: Some(next_debt - 1),
                    sig_pos: self.sig_pos + 1,
                    pos: self.pos + 1,
                }
            }
        };

        Some(ret)
    }
}

#[derive(Copy, Clone, PartialEq, Hash)]
enum Symbol {
    Operational,
    Damaged,
    Unknown,
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

impl TryFrom<char> for Symbol {
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
        "?###???????? 3,2,1",
    ];
    let should_be1 = [1, 4, 1, 1, 4, 10];
    let should_be2 = [1, 16384, 1, 16, 2500, 506250];

    for i in 0..lines.len() {
        assert_eq!(Record::parse(lines[i]).count_solutions(), should_be1[i]);
    }

    for i in 0..lines.len() {
        let rec = Record::parse(lines[i]).unfold();
        assert_eq!(rec.count_solutions(), should_be2[i]);
    }
}
