const INPUT: &str = include_str!("../input/day16");

fn hex_digit_to_bits(d: char) -> Option<&'static [u8]> {
    const DIGITS: [[u8; 4]; 16] = [
        [0, 0, 0, 0], // 0
        [0, 0, 0, 1], // 1
        [0, 0, 1, 0], // 2
        [0, 0, 1, 1], // 3
        [0, 1, 0, 0], // 4
        [0, 1, 0, 1], // 5
        [0, 1, 1, 0], // 6
        [0, 1, 1, 1], // 7
        [1, 0, 0, 0], // 8
        [1, 0, 0, 1], // 9
        [1, 0, 1, 0], // A
        [1, 0, 1, 1], // B
        [1, 1, 0, 0], // C
        [1, 1, 0, 1], // D
        [1, 1, 1, 0], // E
        [1, 1, 1, 1], // F
    ];

    let d = d.to_digit(16)? as usize;

    Some(&DIGITS[d])
}

#[derive(Debug, Clone)]
struct Packet {
    version: u64,
    type_id: u64,
    payload: Payload,
}

#[derive(Debug, Clone)]
enum Payload {
    Literal(u64),
    Subpackets(Packets)
}

// It's somewhat easier, and definitely faster to model the subpackets using a
// vector, but I wanted to try out this rather functional style.
// Mostly because of the idom "make illegal states unrepresentable". If one models
// the subpackets with a vector, then the degenerate state of zero subpackets is representable.
// Here, it is not.

#[derive(Debug, Clone)]
enum Packets {
    One(Box<Packet>),
    More(Box<Packet>, Box<Packets>)
}

impl <'a> IntoIterator for &'a Packets {
    type Item = &'a Packet;
    type IntoIter = PacketsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PacketsIterator {
            current: Some(self)
        }
    }
}

struct PacketsIterator<'a> {
    current: Option<&'a Packets>
}

impl <'a> Iterator for PacketsIterator<'a> {
    type Item = &'a Packet;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current {
            Some(Packets::One(packet)) => {
                self.current = None;

                Some(&packet)
            },
            Some(Packets::More(packet, more)) => {
                self.current = Some(&more);

                Some(&packet)
            },
            _ => None
        }
    }
}

fn parse_number(bits: &[u8], n: usize) -> Option<(u64, &[u8])> {
    let mut res = 0;

    if bits.len() < n {
        return None;
    }

    let (nr, rem) = bits.split_at(n);

    for &b in nr {
        res *= 2;
        res += b as u64;
    }

    Some((res, rem))
}

fn parse_bit(bits: &[u8]) -> Option<(u8, &[u8])> {
    if bits.len() > 0 {
        return Some((bits[0], &bits[1..]));
    } else {
        None
    }
}

fn parse_literal(mut bits: &[u8]) -> Option<(u64, &[u8])> {
    if bits.len() == 0 {
        return None;
    }

    let mut res = 0;

    loop {
        let (read_next, rem) = parse_bit(bits)?;
        bits = rem;

        let (number, rem) = parse_number(bits, 4)?;
        bits = rem;

        res *= 1 << 4;
        res += number;

        if read_next == 0 {
            break;
        }
    }

    Some((res, bits))
}

fn parse_subpackets(bits: &[u8], n: Option<usize>) -> Option<(Packets, &[u8])> {
    if n == Some(0) {
        return None;
    }

    let (packet, rem1) = parse_packet(bits)?;

    if let Some((subpackets, rem2)) = parse_subpackets(rem1, n.map(|n| n - 1)) {
        let sub = Packets::More(packet.into(), subpackets.into());

        Some((sub, rem2))
    } else {
        let sub = Packets::One(packet.into());
        Some((sub, rem1))
    }
}

fn parse_packet(bits: &[u8]) -> Option<(Packet, &[u8])> {
    let (version, bits) = parse_number(bits, 3)?;
    let (type_id, bits) = parse_number(bits, 3)?;

    if type_id == 4 {
        // Literal packet

        let (literal, bits) = parse_literal(bits)?;
        let packet = Packet {
            version,
            type_id,
            payload: Payload::Literal(literal),
        };

        Some((packet, bits))
    } else {
        // Operator packet
        let (lenght_type_id, bits) = parse_bit(bits)?;

        if lenght_type_id == 0 {
            let (length, bits) = parse_number(bits, 15)?;
            let (bits, rem_all) = bits.split_at(length as usize);

            let (subpackets, rem ) = parse_subpackets(bits, None)?;

            if rem.len() != 0 {
                return None;
            }

            let packet = Packet {
                version,
                type_id,
                payload: Payload::Subpackets(subpackets),
            };

            Some((packet, rem_all))
        } else {
            let (no, bits) = parse_number(bits, 11)?;

            let (subpackets, rem) = parse_subpackets(bits, Some(no as usize))?;

            let packet = Packet {
                version,
                type_id,
                payload: Payload::Subpackets(subpackets),
            };

            Some((packet, rem))
        }
    }
}

pub fn solve() -> crate::Result<()> {
    let line = INPUT
        .lines()
        .next()
        .ok_or(crate::Error::NoInput)?;

    let bits = line
        .chars()
        .filter_map(|d| hex_digit_to_bits(d))
        .flatten()
        .copied()
        .collect::<Vec<_>>();

    let bits = bits.as_slice();

    // We only treat the case here, where there is one outermost packet and all other packets
    // are subpackets of it. This seems to be the case (and should be the case because of part 2).

    let (outer_packet, _) = parse_packet(bits).ok_or(crate::Error::InvalidInput)?;

    fn evaluate(p: &Packet) -> u64 {
        match &p.payload {
            Payload::Literal(l) => *l,
            Payload::Subpackets(subpackets) => {
                let mut subpackets = subpackets.into_iter();

                match p.type_id {
                    0 => subpackets.map(|p| evaluate(p)).sum(),
                    1 => subpackets.map(|p| evaluate(p)).product(),
                    2 => subpackets.map(|p| evaluate(p)).min().unwrap(),
                    3 => subpackets.map(|p| evaluate(p)).max().unwrap(),
                    5 => {
                        // Greater than
                        let p1 = subpackets.next().unwrap();
                        let p2 = subpackets.next().unwrap();

                        (evaluate(p1) > evaluate(p2)) as u64
                    }
                    6 => {
                        // Less than
                        let p1 = subpackets.next().unwrap();
                        let p2 = subpackets.next().unwrap();
                        
                        (evaluate(p1) < evaluate(p2)) as u64
                    }
                    7 => {
                        // Equal
                        let p1 = subpackets.next().unwrap();
                        let p2 = subpackets.next().unwrap();

                        (evaluate(p1) == evaluate(p2)) as u64
                    },
                    _ => panic!("invalid type id")
                }
            }
        }
    }

    fn version_sum(p: &Packet) -> u64 {
        match &p.payload {
            &Payload::Literal(_) => p.version,
            Payload::Subpackets(subpackets) => {
                p.version + (&subpackets).into_iter()
                    .map(|p| version_sum(p))
                    .sum::<u64>()
            }
        }
    }

    println!("Problem 1: {}", version_sum(&outer_packet));
    println!("Problem 2: {}", evaluate(&outer_packet));

    Ok(())
}
