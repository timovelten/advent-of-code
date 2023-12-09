const INPUT: &str = include_str!("../input/day09.txt");

pub fn solve() -> crate::Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in INPUT.lines() {
        let numbers = line.split_ascii_whitespace()
            .map(|x| x.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        let (forward, backward) = extrapolate(&numbers);
        part1 += forward;
        part2 += backward;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

fn extrapolate(data: &[i64]) -> (i64, i64) {
    let (first, last) = (*data.first().unwrap(), *data.last().unwrap());
    let mut differences = vec![0; data.len() - 1];
    let mut all_zero = last == 0;
    for i in 0..data.len() - 1 {
        differences[i] = data[i + 1] - data[i];
        all_zero &= data[i] == 0;
    }

    if !all_zero {
        let (forward, backward) = extrapolate(&differences);
        (last + forward, first - backward)
    } else {
        (0, 0)
    }
}

#[test]
fn example() {
    assert_eq!(extrapolate(&[0, 3, 6, 9, 12, 15]).0, 18);
    assert_eq!(extrapolate(&[1, 3, 6, 10, 15, 21]).0, 28);
    assert_eq!(extrapolate(&[10, 13, 16, 21, 30, 45]).0, 68);
    assert_eq!(extrapolate(&[10, 13, 16, 21, 30, 45]).1, 5);
}
