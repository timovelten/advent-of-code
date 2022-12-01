use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day17");

fn parse_range(s: &str) -> Option<(i64, i64)> {
    let mut split = s.split("..");

    let start = split.next()?.parse::<i64>().ok()?;
    let end = split.next()?.parse::<i64>().ok()?;

    Some((start, end))
}

pub fn solve() -> crate::Result<()> {
    let line = INPUT
        .lines()
        .next()
        .ok_or(crate::Error::NoInput)?;

    let line = &line["target area: ".len()..];
    let mut split = line.split(", ");

    let x_range = &split.next().ok_or(crate::Error::InvalidInput)?["x=".len()..];
    let y_range = &split.next().ok_or(crate::Error::InvalidInput)?["x=".len()..];

    let (start_x, end_x): (i64, i64) = parse_range(x_range).ok_or(crate::Error::InvalidInput)?;
    let (start_y, end_y): (i64, i64) = parse_range(y_range).ok_or(crate::Error::InvalidInput)?;

    let mut possible_velocities: HashSet<(i64, i64)> = Default::default();

    for y in start_y..=end_y {
        // Determine all the (t, v0_y) such that the probe will
        // land in y at step t.

        // Note that if v0 is some velocity, and v(t) = v0 - t, then
        // s(t) = \sum_{i = 0}^(t - 1) v(i) = t v0 - 0.5 t (t - 1)
        // so
        // 2 v0 = t - 1 + 2 s(t) / t
        // From this, we see that t is a divisor 2 s(t), i.e. t <= 2 |y| and that
        // v0 = 1/2 (t - 1 + 2 s(t) / t)
        for t in 1..=2 * y.abs() {
            // Note the parantheses, they are (unfortunately) important so that 
            // the integer divison works out the way we need it to.
            let v0_y = (t - 1 + (2 * y) / t) / 2;
            let sy = t * v0_y - (t * t - t) / 2;
            
            if sy != y {
                // The probe does not actually land at y with this (t, v0_y)
                continue;
            }

            // Now, try and find a matching v0_x
            for v0_x in 0..=end_x {
                let sx = if v0_x >= t {
                    v0_x * t - (t * t - t) / 2
                } else {
                    // We have reached "terminal velocity" and are therefore not moving anymore
                    (v0_x * v0_x + v0_x) / 2
                };

                if start_x <= sx && sx <= end_x {
                    possible_velocities.insert((v0_x, v0_y));
                }
            }
        }
    }

    let max_y_velocity = possible_velocities
        .iter()
        .map(|(_, vy)| (vy * vy + vy) / 2)
        .max();

    println!("Problem 1: {}", max_y_velocity.unwrap());
    println!("Problem 2: {}", possible_velocities.len());

    Ok(())
}
