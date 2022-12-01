use crate::Error;

const INPUT: &str = include_str!("../input/day2");

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl TryFrom<&str> for Instruction {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split_ascii_whitespace();
        let first = split.next().ok_or(Error::InvalidInput)?;
        let second = split.next().ok_or(Error::InvalidInput)?;

        let nr = second.parse()?;

        match first {
            "forward" => Ok(Instruction::Forward(nr)),
            "down" => Ok(Instruction::Down(nr)),
            "up" => Ok(Instruction::Up(nr)),
            _ => Err(Error::InvalidInput),
        }
    }
}

pub fn solve() -> Result<(), crate::Error> {
    let instructions = INPUT
        .lines()
        .filter_map(|s| Instruction::try_from(s).ok())
        .collect::<Vec<Instruction>>();

    // Part 1
    let mut pos = 0;
    let mut depth = 0;

    for &inst in &instructions {
        match inst {
            Instruction::Down(x) => depth += x,
            Instruction::Up(x) => depth -= x,
            Instruction::Forward(x) => pos += x,
        }
    }

    println!("Problem 1: {}", pos * depth);

    // Part 2
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for inst in instructions {
        match inst {
            Instruction::Down(x) => aim += x,
            Instruction::Up(x) => aim -= x,
            Instruction::Forward(x) => {
                pos += x;
                depth += aim * x;
            }
        }
    }

    println!("Problem 2: {}", pos * depth);

    Ok(())
}
