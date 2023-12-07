const INPUT: &str = include_str!("../input/day05.txt");

pub fn solve() -> crate::Result<()> {
    let (seeds, maps) = parse(INPUT)?;
    let (part1, part2) = solve_impl(seeds, maps);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

fn solve_impl(seeds: Vec<i64>, maps: Vec<IntervalCollection<i64>>) -> (i64, i64) {
    let mut part1 = i64::MAX;
    for &seed in &seeds {
        let mut res = seed;
        for map in &maps {
            res = eval(map, res);
        }

        part1 = part1.min(res);
    }

    let mut res2 = IntervalCollection::new();
    for chunk in seeds.chunks_exact(2) {
        let start = chunk[0];
        let end = chunk[0] + chunk[1] - 1;
        res2.add(Interval {
            start,
            end,
            label: (),
        });
    }

    for map in &maps {
        let mut next_res = IntervalCollection::new();
        for y in res2.elems {
            next_res.add_all(image(map, y));
        }
        res2 = next_res;
    }

    (part1, res2.min())
}

fn parse(s: &str) -> crate::Result<(Vec<i64>, Vec<IntervalCollection<i64>>)> {
    let mut blocks = s.split("\n\n");
    let seeds = blocks
        .next()
        .ok_or("expected input")?
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<_>>();

    let mut ret = Vec::new();
    for block in blocks {
        let mut map = IntervalCollection::new();
        for line in block.lines().skip(1) {
            let [dst, src, len] = parse_numbers(line)?;
            let interval = Interval {
                start: src,
                end: src + len - 1,
                label: dst - src,
            };
            map.add(interval);
        }
        ret.push(map);
    }

    Ok((seeds, ret))
}

fn parse_numbers<const N: usize>(s: &str) -> crate::Result<[i64; N]> {
    let mut ret = [0; N];
    let mut nos = s
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<i64>().ok());

    for item in ret.iter_mut().take(N) {
        *item = nos.next().ok_or("expected number")?;
    }

    Ok(ret)
}

fn eval(f: &IntervalCollection<i64>, x: i64) -> i64 {
    let res = image(
        f,
        Interval {
            start: x,
            end: x,
            label: (),
        },
    );
    assert!(res.elems.len() == 1);
    res.elems[0].start
}

fn image(f: &IntervalCollection<i64>, x: Interval<()>) -> IntervalCollection<()> {
    let mut offsets = IntervalCollection::new();
    offsets.add(Interval {
        start: x.start,
        end: x.end,
        label: 0,
    });
    for y in &f.elems {
        let clamped_y = y.clamp(Some(x.start), Some(x.end));
        offsets.add(clamped_y);
    }

    let mut ret = IntervalCollection::new();
    for y in offsets.elems {
        let mapped_y = Interval {
            start: y.start + y.label,
            end: y.end + y.label,
            label: (),
        };

        ret.add(mapped_y);
    }

    ret.simplify();
    ret
}

#[derive(Clone, PartialEq, Debug)]
struct Interval<T = ()> {
    start: i64,
    end: i64,
    label: T,
}

impl<T: Clone> Interval<T> {
    fn is_trivial(&self) -> bool {
        self.start > self.end
    }

    fn clamp(&self, start: Option<i64>, end: Option<i64>) -> Interval<T> {
        let start = if let Some(start) = start {
            start.max(self.start)
        } else {
            self.start
        };

        let end = if let Some(end) = end {
            end.min(self.end)
        } else {
            self.end
        };

        Interval {
            start,
            end,
            label: self.label.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct IntervalCollection<T> {
    /// The intervals are disjoint, and ordered by their endpoints (and then also automatically
    /// by their starting points)
    elems: Vec<Interval<T>>,
}

impl<T: Clone + std::fmt::Debug + std::cmp::PartialEq> IntervalCollection<T> {
    fn simplify(&mut self) {
        // Note that we iterate in reverse order so that the changing indices (we remove elements)
        // do not affect us
        for i in (0..self.elems.len() - 1).rev() {
            if self.elems[i].label == self.elems[i + 1].label
                && self.elems[i].end + 1 == self.elems[i + 1].start
            {
                self.elems[i].end = self.elems[i + 1].end;
                self.elems.remove(i + 1);
            }
        }
    }

    fn add(&mut self, mut range: Interval<T>) {
        // Note: the hard part here is to keep the intervals disjoint
        while !range.is_trivial() {
            if let Some(next_idx) = self.get_next(range.start) {
                let existing = &mut self.elems[next_idx];
                assert!(range.start <= existing.end);

                // In the following "pictures" x is range,
                // and y is the existing interval.
                if range.end < existing.start {
                    // The two intervals are disjoint
                    self.elems.insert(next_idx, range);
                    break;
                } else if range.start <= existing.start && existing.end <= range.end {
                    // Existing interval is contained entirely in the new range,
                    // so we can replace it
                    let next_start = existing.end + 1;
                    self.elems[next_idx] = Interval {
                        start: range.start,
                        end: existing.end,
                        label: range.label.clone(),
                    };
                    range.start = next_start;
                } else if range.start <= existing.start {
                    // ... then existing.end > range.end:
                    // xxxxx
                    //   yyyyy
                    existing.start = range.end + 1;
                    self.elems.insert(next_idx, range.clone());
                    break;
                } else if existing.end <= range.end {
                    // ...then range.start > existing.start
                    //   xxxx
                    // yyyy
                    let next_start = existing.end + 1;
                    let clamped_range = Interval {
                        start: range.start,
                        end: existing.end,
                        label: range.label.clone(),
                    };
                    existing.end = range.start - 1;
                    self.elems.insert(next_idx + 1, clamped_range);
                    range.start = next_start;
                } else {
                    //   xxxx
                    // yyyyyyyy
                    let left = Interval {
                        start: existing.start,
                        end: range.start - 1,
                        label: existing.label.clone(),
                    };

                    let right = Interval {
                        start: range.end + 1,
                        end: existing.end,
                        label: existing.label.clone(),
                    };

                    *existing = right;
                    self.elems.insert(next_idx, range);
                    self.elems.insert(next_idx, left);
                    break;
                }
            } else {
                // range needs to become the largest interval
                self.elems.push(range);
                break;
            }
        }
    }

    fn add_all(&mut self, other: IntervalCollection<T>) {
        for y in other.elems {
            self.add(y);
        }
    }

    fn min(&self) -> i64 {
        self.elems[0].start
    }

    /// This returns the "next" interval starting from x in the following sense:
    /// 1) If x is contained in any interval, that (unique!) interval is returned
    /// 2) Otherwise, we look for the next END interval boundary. When we encounter it,
    ////   that is the interval we return
    fn get_next(&self, x: i64) -> Option<usize> {
        let idx = self
            .elems
            .binary_search_by_key(&x, |x| x.end)
            .unwrap_or_else(|i| i);

        if idx < self.elems.len() {
            return Some(idx);
        }

        None
    }

    fn new() -> Self {
        Self {
            elems: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4     
    "};

    #[test]
    fn test_example() {
        let (seeds, maps) = parse(EXAMPLE).unwrap();
        let (part1, part2) = solve_impl(seeds, maps);

        assert_eq!(part1, 35);
        assert_eq!(part2, 46);
    }
}
