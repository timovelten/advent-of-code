const INPUT: &str = include_str!("../input/day05.txt");

pub fn solve() -> crate::Result<()> {
    let (seeds, maps) = parse(INPUT)?;

    let mut part1 = i64::MAX;
    for seed in seeds {
        let mut res = seed;
        for map in &maps {
            res = map.eval(res);
        }
       
        part1 = part1.min(res);
    }

    println!("Part 1: {part1}");

    Ok(())
}

fn parse(s: &str) -> crate::Result<(impl Iterator<Item=i64> + '_, Vec<Map>)> {
    let mut blocks = s.split("\n\n");
    let seeds = blocks.next()
        .ok_or("expected input")?
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok());

    let mut ret = Vec::new();
    for block in blocks {
        let map = Map::parse(block)?;
        ret.push(map);
    }

    Ok((seeds, ret))
}

/// Functions that can be represenated by this class are of the form
///     f = id + off_1*1_{I_1} + off_2*1_{I_2} +
/// where 1_{I_1}(x) is the indicator function of some interval I_1. The intervals I_1, I_2, ...
/// are disjoint.
#[derive(Debug)]
struct Map {
    /// The entries are guaranteed to never overlap (i.e. for any two entries entrt.src_range() are guaranteed to be disjoint)
    /// Also, this list is sorted by entry.end (and also by entry.start, as the ranges do not overlap)
    ranges: Vec<SimpleMap>,
}

impl Map {
    fn new() -> Self {
        Self {
            ranges: Default::default(),
        }
    }

    fn parse(s: &str) -> crate::Result<Self> {
        let lines = s.lines().skip(1);
        let mut ret = Self::new();

        for line in lines {
            let mut nos = line
                .split_ascii_whitespace()
                .filter_map(|x| x.parse::<i64>().ok());

            let dst_start = nos.next().ok_or("expected number")?;
            let src_start = nos.next().ok_or("expected number")?;
            let len = nos.next().ok_or("expected number")?;

            let entry = SimpleMap::new(dst_start, src_start, len);
            ret.add(entry);
        }

        Ok(ret)
    }

    fn add(&mut self, new_entry: SimpleMap) {
        // Take care to insert at the right index
        let res = self
            .ranges
            .binary_search_by_key(&new_entry.end, |entry| entry.end);
        let idx = match res {
            Ok(i) | Err(i) => i,
        };

        let conflict = if let Some(entry) = self.ranges.get(idx) {
            entry.intersects(&new_entry)
        } else {
            false
        };

        if conflict {
            panic!("range may not intersect an existing range")
        } else {
            self.ranges.insert(idx, new_entry);
        }
    }


    fn eval(&self, x: i64) -> i64 {
        let value = self.ranges.binary_search_by_key(&x, |entry| entry.end);
        let idx = match value {
            Ok(i) | Err(i) => {
                // If this errors, it returns the element where we WOULD need to be inserted for the list to remain sorted
                // (i.e. the element it returns then is >= x). This is why we look for the end of a range, instead of the start.
                i
            }
        };

        // At most one interval can apply to this value
        let mut ret = x;

        if let Some(entry) = self.ranges.get(idx) {
            ret += entry.eval(x);
        }

        ret
    }
}

/// This represents the map offset * 1_I 
/// (indicator function of the interval I multiplied by some constant),
#[derive(Debug, PartialEq, Clone)]
struct SimpleMap {
    start: i64,
    end: i64,
    alpha: i64
}

impl SimpleMap {    
    fn new(dst_start: i64, src_start: i64, len: i64) -> Self {
        let start = src_start;
        let end = src_start + len - 1;
        let offset = dst_start - src_start;
        Self { start, end, alpha: offset }
    }
    
    fn eval(&self, x: i64) -> i64 {
        if x >= self.start && x <= self.end {
            return self.alpha;
        }

        0
    }

    fn intersects(&self, other: &SimpleMap) -> bool {
        let (first, second) = if self.start <= other.start {
            (self, other)
        } else {
            (other, self)
        };

        second.start <= first.end
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
    fn very_basic_test() {
        let mut map = Map::new();
        map.add(SimpleMap::new(50, 98, 2));
        map.add(SimpleMap::new(52, 50, 48));
        assert_eq!(map.eval(98), 50);
        assert_eq!(map.eval(99), 51);
        assert_eq!(map.eval(79), 81);
        assert_eq!(map.eval(14), 14);
        assert_eq!(map.eval(55), 57);
        assert_eq!(map.eval(13), 13);
    }

    #[test]
    fn parse_test() {
        let example = indoc! {"
            seed-to-soil map:
            50 98 2
            52 50 48
        "};
        let map = Map::parse(example).unwrap();

        assert_eq!(map.eval(98), 50);
        assert_eq!(map.eval(99), 51);
        assert_eq!(map.eval(79), 81);
        assert_eq!(map.eval(14), 14);
        assert_eq!(map.eval(55), 57);
        assert_eq!(map.eval(13), 13);
    }

    #[test]
    fn complex_test() {
        let (seeds, maps) = parse(EXAMPLE).unwrap();

        let results: Vec<_> = seeds
            .map(|seed| {
                let mut res = seed;
                for map in &maps {
                    res = map.eval(res);
                }

                res
            })
            .collect();

        assert_eq!(results, [82, 43, 86, 35]);
    }

    #[test]
    fn part2() {
        
    }
}
