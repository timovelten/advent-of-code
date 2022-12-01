use ahash::AHashMap;

const INPUT: &str = include_str!("../input/day24");

type T = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Value),
    Mul(Variable, Value),
    Div(Variable, Value),
    Mod(Variable, Value),
    Eql(Variable, Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Literal(T),
    Variable(Variable),
}

impl Value {
    fn get(&self, mem: &[T; 4]) -> T {
        match &self {
            Self::Literal(x) => *x,
            Self::Variable(v) => v.get(mem),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variable {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}

impl Variable {
    fn get(&self, mem: &[T; 4]) -> T {
        mem[*self as usize]
    }

    fn get_mut<'a>(&self, mem: &'a mut [T; 4]) -> &'a mut T {
        &mut mem[*self as usize]
    }
}

fn parse_literal(s: &str) -> Option<(T, &str)> {
    let digit_count = s
        .chars()
        .take_while(|c| c.is_digit(10) || *c == '-')
        .count();
    let literal = s[..digit_count].parse::<T>().ok()?;

    Some((literal, &s[digit_count..]))
}

fn parse_variable(s: &str) -> Option<(Variable, &str)> {
    if s.len() == 0 {
        return None;
    }

    let (c, rem) = s.split_at(1);
    let var = match c {
        "w" => Variable::W,
        "x" => Variable::X,
        "y" => Variable::Y,
        "z" => Variable::Z,
        _ => return None,
    };

    Some((var, rem))
}

fn parse_value(s: &str) -> Option<(Value, &str)> {
    if let Some((var, rem)) = parse_variable(s) {
        Some((Value::Variable(var), rem))
    } else if let Some((l, rem)) = parse_literal(s) {
        Some((Value::Literal(l), rem))
    } else {
        None
    }
}

fn parse_instruction(s: &str) -> Option<(Instruction, &str)> {
    if s.starts_with("inp ") {
        let (a, rem) = parse_variable(&s[4..])?;

        Some((Instruction::Inp(a), rem))
    } else if s.starts_with("add ") {
        let (a, rem) = parse_variable(&s[4..])?;
        let (b, rem) = parse_value(&rem[1..])?;

        Some((Instruction::Add(a, b), rem))
    } else if s.starts_with("mul ") {
        let (a, rem) = parse_variable(&s[4..])?;
        let (b, rem) = parse_value(&rem[1..])?;

        Some((Instruction::Mul(a, b), rem))
    } else if s.starts_with("div ") {
        let (a, rem) = parse_variable(&s[4..])?;
        let (b, rem) = parse_value(&rem[1..])?;

        Some((Instruction::Div(a, b), rem))
    } else if s.starts_with("mod ") {
        let (a, rem) = parse_variable(&s[4..])?;
        let (b, rem) = parse_value(&rem[1..])?;

        Some((Instruction::Mod(a, b), rem))
    } else if s.starts_with("eql ") {
        let (a, rem) = parse_variable(&s[4..])?;
        let (b, rem) = parse_value(&rem[1..])?;

        Some((Instruction::Eql(a, b), rem))
    } else {
        None
    }
}

/// Run the given until the second input instruction is hit and return the remaining program
fn run<'a>(instructions: &'a [Instruction], mem: &mut [T; 4], input: T) -> &'a [Instruction] {
    let mut input = Some(input);

    let mut pc = 0;

    while pc < instructions.len() {
        let inst = instructions[pc];

        match inst {
            Instruction::Inp(a) => {
                if let Some(inp) = input.take() {
                    *a.get_mut(mem) = inp;
                } else {
                    break;
                }
            }
            Instruction::Add(a, b) => {
                *a.get_mut(mem) += b.get(mem);
            }
            Instruction::Mul(a, b) => {
                *a.get_mut(mem) *= b.get(mem);
            }
            Instruction::Div(a, b) => {
                *a.get_mut(mem) /= b.get(mem);
            }
            Instruction::Mod(a, b) => {
                let b = b.get(mem);
                let v = a.get_mut(mem);

                *v = v.rem_euclid(b);
            }
            Instruction::Eql(a, b) => {
                *a.get_mut(mem) = (a.get(mem) == b.get(&mem)) as T;
            }
        }

        pc += 1;
    }

    &instructions[pc..]
}

fn find_solution(
    program: &[Instruction],
    depth: u8,
    z: T,
    cache: &mut AHashMap<(u8, T), Option<u64>>,
    biggest: bool,
) -> Option<u64> {
    if program.len() == 0 {
        return if z == 0 { Some(0) } else { None };
    }

    // (*)
    if let Some(c) = 27u64.checked_pow(14 - depth as u32) {
        if z as u64 > c {
            return None;
        }
    }

    if let Some(&ret) = cache.get(&(depth, z)) {
        return ret;
    }

    let mut ret = None;

    let input = if biggest {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };

    for &w in &input {
        let mut mem = [0, 0, 0, 0];
        mem[Variable::Z as usize] = z;

        let remaining_program = run(&program, &mut mem, w);

        let nz = mem[Variable::Z as usize];

        if let Some(val) = find_solution(remaining_program, depth + 1, nz, cache, biggest) {
            ret = Some(10 * val + (w as u64));
            break;
        }
    }

    cache.insert((depth, z), ret);

    ret
}

fn reverse_10(mut u: u64) -> u64 {
    let mut ret = 0;

    while u != 0 {
        ret *= 10;
        ret += u % 10;
        u /= 10;
    }

    ret
}

pub fn solve() -> crate::Result<()> {
    let instructions = INPUT
        .lines()
        .map(|line| parse_instruction(&line).map(|(i, _)| i))
        .collect::<Option<Vec<Instruction>>>()
        .ok_or(crate::Error::InvalidInput)?;

    // This solution tries to be as general as possible, never the less, we assume a few things
    // about the input:
    // - w, x, y are just used as temporaries and do not matter across different inp instructions
    // - there are 14 inp instructions
    // Finally, there is an optimization (*), which we could leave out and the program
    // would still work - just much slower - that assumes
    // - in each block (by which we mean a section starting with inp and otherwise
    // having no other inp's) the variable z can only shrink by a factor of 27  i.e.
    // if z0 is before the block is run, and z1 is afterwards, we assume that 27 * z0 >= z1

    println!(
        "Problem 1: {:?}",
        find_solution(&instructions, 0, 0, &mut Default::default(), true)
            .map(reverse_10)
            .unwrap()
    );

    println!(
        "Problem 2: {:?}",
        find_solution(&instructions, 0, 0, &mut Default::default(), false)
            .map(reverse_10)
            .unwrap()
    );

    Ok(())
}
