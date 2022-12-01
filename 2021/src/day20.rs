use std::{cmp::{max, min}, fmt::Display};

use ahash::AHashSet;

const INPUT: &str = include_str!("../input/day20");

#[derive(Debug, Clone)]
struct Image {
    pixels: AHashSet<(i16, i16)>,
    x_min: i16,
    x_max: i16,
    y_min: i16,
    y_max: i16,
    edge_lit: bool,
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.y_min..=self.y_max {
            for x in self.y_min..=self.y_max {
                if self.pixels.contains(&(y, x)) {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Image {
    fn is_lit(&self, x: i16, y: i16) -> bool {
        if self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max {
            self.pixels.contains(&(y, x))
        } else {
            self.edge_lit
        }
    }

    fn enhance(&self, algorithm: &[bool]) -> Self {
        let edge_lit = if self.edge_lit {
            // Edge is lit, so every 9 x 9 group makes only ones
            algorithm[0b111111111]
        } else {
            algorithm[0]
        };

        // Try and reduce the size of the bounding box to save on iterations
        let (mut y_min, mut y_max) = (std::i16::MAX, std::i16::MIN);
        let (mut x_min, mut x_max) = (std::i16::MAX, std::i16::MIN);

        let mut pixels: AHashSet<(i16, i16)> = Default::default();
        for x in (self.x_min - 3)..=(self.x_max + 3) {
            for y in (self.y_min - 3)..=(self.y_max + 3) {
                let mut idx = 0;

                // Note that the order here is very much relevant
                for (dy, dx) in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 0),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    idx *= 2;
                    idx += self.is_lit(x + dx, y + dy) as usize;
                }

                let lit = algorithm[idx];
                if lit {
                    pixels.insert((y, x));
                }

                if lit != edge_lit {
                    y_min = min(y_min, y);
                    y_max = max(y_max, y);
                    x_min = min(x_min, x);
                    x_max = max(x_max, x);
                }
            }
        }

        // Degenerate case, that will never actually occur: It could happen that
        // we never actually change the values of these variables.
        if y_min == std::i16::MAX {
            y_min = self.y_min;
            y_max = self.y_min;
            x_min = self.x_min;
            x_max = self.x_min;
        }


        Self {
            pixels,
            y_min,
            y_max,
            x_min,
            x_max,
            edge_lit,
        }
    }

    fn count_pixels(&self) -> Option<usize> {
        if !self.edge_lit {
            Some(self.pixels.len())
        } else {
            None
        }
    }
}

fn parse_image<S: AsRef<str>, I: IntoIterator<Item = S>>(lines: I) -> Image {
    let mut pixels: AHashSet<(i16, i16)> = Default::default();

    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in lines.into_iter().enumerate() {
        let line = line.as_ref();

        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                pixels.insert((y as i16, x as i16));
            }

            max_x = max(max_x, x);
        }

        max_y = max(max_y, y);
    }

    Image {
        pixels,
        x_min: 0,
        x_max: max_x as i16,
        y_min: 0,
        y_max: max_y as i16,
        edge_lit: false,
    }
}

pub fn solve() -> crate::Result<()> {
    let mut lines = INPUT.lines();

    let algorithm = lines
        .next()
        .ok_or(crate::Error::NoInput)?
        .chars()
        .map(|x| x == '#')
        .collect::<Vec<_>>();

    // Skip empty line
    let _ = lines.next();

    let img = parse_image(lines);

    println!(
        "Problem 1: {}",
        img.enhance(&algorithm)
            .enhance(&algorithm)
            .count_pixels()
            .unwrap()
    );

    let mut enhanced = img;
    for _ in 0..50 {
        enhanced = enhanced.enhance(&algorithm);
    }

    println!("Problem 2: {}", enhanced.count_pixels().unwrap());

    Ok(())
}
