use std::{
    ops::Sub,
};

use ahash::{AHashMap, AHashSet};
use lazy_static::lazy_static;

const INPUT: &str = include_str!("../input/day19");

// This is a re-implementation of the python version.
// See the Python version for more detailed comments.

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
struct Point(i32, i32, i32);

impl Point {
    fn l1(&self) -> u32 {
        let &Point(p1, p2, p3) = self;
        p1.unsigned_abs() + p2.unsigned_abs() + p3.unsigned_abs()
    }

    fn component(&self, i: usize) -> i32 {
        match i {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            _ => panic!("out of bounds"),
        }
    }

    fn rotate(&self, (s1, s2, s3, (i, j, k)): Rotation) -> Point {
        Point(
            s1 * self.component(i),
            s2 * self.component(j),
            s3 * self.component(k),
        )
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, Point(b1, b2, b3): Self) -> Self::Output {
        let Point(a1, a2, a3) = self;
        Point(a1 - b1, a2 - b2, a3 - b3)
    }
}

type Permutation = (usize, usize, usize);
type Rotation = (i32, i32, i32, Permutation);

const S3_WITH_SIGNS: [(i32, Permutation); 6] = [
    (1, (0, 1, 2)),
    (1, (1, 2, 0)),
    (1, (2, 0, 1)),
    (-1, (1, 0, 2)),
    (-1, (0, 2, 1)),
    (-1, (2, 1, 0)),
];

lazy_static! {
    static ref ROTATIONS: Vec<Rotation> = {
        let mut ret = Vec::new();

        for &(s, perm) in &S3_WITH_SIGNS {
            for s1 in [-1, 1] {
                for s2 in [-1, 1] {
                    for s3 in [-1, 1] {
                        if s * s1 * s2 * s3 == -1 {
                            continue;
                        }

                        ret.push((s1, s2, s3, perm))
                    }
                }
            }
        }

        ret
    };
}

struct BeaconSet {
    points_with_distances: AHashMap<Point, AHashSet<u32>>,
}

impl BeaconSet {
    fn from(set: AHashSet<Point>) -> Self {
        let mut points_with_distances: AHashMap<Point, AHashSet<u32>> = Default::default();

        for &x in &set {
            let x_distances = points_with_distances.entry(x).or_default();

            for &y in &set {
                x_distances.insert((x - y).l1());
            }
        }

        BeaconSet {
            points_with_distances,
        }
    }

    fn rotate_and_translate(&mut self, rot: Rotation, d: Point) {
        let n = self.points_with_distances.len();

        let old = std::mem::replace(&mut self.points_with_distances, AHashMap::with_capacity(n));

        for (x, distances) in old {
            let new_x = x.rotate(rot) - d;
            self.points_with_distances.insert(new_x, distances);
        }
    }
}

fn find_rotation_and_offset(set1: &BeaconSet, set2: &BeaconSet) -> Option<(Rotation, Point)> {
    for (&p, distances_p) in &set1.points_with_distances {
        for (&q, distances_q) in &set2.points_with_distances {
            if distances_p.intersection(distances_q).count() < 12 {
                continue;
            }

            for &rot in ROTATIONS.iter() {
                let d = q.rotate(rot) - p;

                let mut m = 0;
                for (r, _) in &set2.points_with_distances {
                    let t_and_r = r.rotate(rot)  - d;
                    if set1.points_with_distances.contains_key(&t_and_r) {
                        m += 1;
                    }
                }

                if m >= 12 {
                    return Some((rot, d));
                }
            }
        }
    }

    None
}

pub fn solve() -> crate::Result<()> {
    let lines = INPUT.lines().chain(std::iter::once("".into()));

    // Assume that the scanners are numbered 0,1,2,3... (without any holes)
    let mut scanners: Vec<BeaconSet> = Default::default();

    let mut current = None;
    let mut point_set: AHashSet<Point> = Default::default();

    for line in lines {
        let line = line.trim();

        if line.starts_with("---") {
            let mut split = line.split_ascii_whitespace();

            let nr = split.nth(2).ok_or(crate::Error::InvalidInput)?;
            current = Some(nr.parse::<usize>()?);
        } else if line.is_empty() {
            scanners.insert(
                current.ok_or(crate::Error::InvalidInput)?,
                BeaconSet::from(point_set),
            );
            point_set = Default::default();
        } else {
            let mut split = line.split(',');

            let x1 = split
                .next()
                .ok_or(crate::Error::InvalidInput)?
                .parse::<i32>()?;
            let x2 = split
                .next()
                .ok_or(crate::Error::InvalidInput)?
                .parse::<i32>()?;
            let x3 = split
                .next()
                .ok_or(crate::Error::InvalidInput)?
                .parse::<i32>()?;

            point_set.insert(Point(x1, x2, x3));
        }
    }

    let n = scanners.len();

    let mut scanner_offsets: AHashMap<usize, Point> = Default::default();
    scanner_offsets.insert(0, Point(0, 0, 0));

    while scanner_offsets.len() < scanners.len() {
        for t in 0..n {
            if scanner_offsets.contains_key(&t) {
                continue;
            }

            for &s in scanner_offsets.keys() {
                if let Some((rot, d)) = find_rotation_and_offset(&scanners[s], &scanners[t]) {
                    scanners[t].rotate_and_translate(rot, d);
                    scanner_offsets.insert(t, d);

                    break;
                }
            }
        }
    }

    // Assemble list of all beacons
    let mut all_beacons = AHashSet::new();
    for s in &scanners {
        for (&x, _) in &s.points_with_distances {
            all_beacons.insert(x);
        }
    }

    println!("Problem 1: {}", all_beacons.len());

    let mut max_distance = 0;
    for s in 0..n {
        for t in 0..n {
            if s == t {
                continue;
            }

            max_distance = std::cmp::max(
                (scanner_offsets[&s] - scanner_offsets[&t]).l1(),
                max_distance,
            )
        }
    }

    println!("Problem 2: {}", max_distance);

    Ok(())
}
