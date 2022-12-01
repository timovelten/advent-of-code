const INPUT: &str = include_str!("../input/day3");

pub fn parse_bitstring(s: &str) -> crate::Result<Vec<u8>> {
    let res = s
        .chars()
        .map(|x| x.to_digit(2).map(|val| val as u8))
        .collect::<Option<Vec<u8>>>();

    res.ok_or(crate::Error::InvalidInput)
}

pub fn solve() -> crate::Result<()> {
    let xs = INPUT.lines()
        .map(|line| parse_bitstring(&line))
        .collect::<crate::Result<Vec<_>>>()?;

    println!("Problem 1: {}", solve1(&xs)?);
    println!("Problem 2: {}", solve2(&xs)?);

    Ok(())
}

fn most_common_bit<I: IntoIterator<Item=u8>>(bits: I) -> u8 {
    let mut res: isize = 0;

    for bit in bits {
        if bit == 1 {
            res += 1;
        } else {
            res -= 1;
        }
    }

    if res >= 0 {
        return 1;
    } else {
        return 0;
    }
}

fn bits_to_number<I: IntoIterator<Item=u8>>(bits: I) -> u64 {
    let mut res: u64 = 0;

    for bit in bits {
        res *= 2;
        res += bit as u64
    }

    res
}

fn solve1(xs: &Vec<Vec<u8>>) -> crate::Result<u64> {
    let n = xs.get(0).ok_or(crate::Error::NoInput)?.len();
    
    let most_common_bits = (0..n)
        .map(|i| most_common_bit(xs.iter().map(|x| x[i])));

    let gamma = bits_to_number(most_common_bits);
    let epsilon = (1 << n) - 1 - gamma;

    return Ok(gamma * epsilon);
}

fn select(xs: &Vec<Vec<u8>>, select_most_common: bool) -> crate::Result<u64> {
    let n = xs.get(0).ok_or(crate::Error::NoInput)?.len();

    let mut selected = vec![true; xs.len()];
    let mut selected_count = xs.len();

    for i in 0..n {
        let selected_xs = xs.iter().enumerate()
            .filter(|&(i, _)| selected[i])
            .map(|(_, x)| x);
        let most_common = most_common_bit(selected_xs.map(|x| x[i]));

        let v = if select_most_common {
            most_common
        } else {
            1 - most_common
        };

        for j in 0..xs.len() {
            if selected[j] && xs[j][i] != v {
                selected[j] = false;
                selected_count -= 1;
            }

            if selected_count <= 1 {
                // Find the first (and only...) still selected number
                let selected_number = xs.iter()
                    .enumerate()
                    .filter(|&(i, _)| selected[i])
                    .map(|(_, x)| x)
                    .next()
                    .ok_or(crate::Error::InvalidInput)?;

                let ret = bits_to_number(selected_number.iter().copied());

                return Ok(ret);
            }
        }
    }

    Err(crate::Error::InvalidInput)
}

fn solve2(xs: &Vec<Vec<u8>>) -> crate::Result<u64> {
    let oxygen_generator_rating = select(xs, true)?;
    let c02_scrubber_rating = select(xs, false)?;

    Ok(oxygen_generator_rating * c02_scrubber_rating)
}
