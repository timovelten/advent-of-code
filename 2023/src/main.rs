use std::{io, num::ParseIntError};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() -> crate::Result<()> {
    const ONLY_DAY: Option<usize> = Some(9);
    #[rustfmt::skip]
    let solvers = [
        day01::solve, 
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve
    ];

    for (i, solver) in solvers.into_iter().enumerate() {
        let day = i + 1;
        if ONLY_DAY.map(|x| x != day).unwrap_or(false) {
            continue;
        }
        println!("Day {}", day);

        let now = std::time::Instant::now();
        solver()?;
        let took = now.elapsed();

        println!("Took {} ms ({} ns)", took.as_millis(), took.as_nanos());
        println!()
    }

    Ok(())
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ParseIntError(ParseIntError),
    NoInput,
    InvalidInput,
    Message(String),
}

type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Error::IOError(val)
    }
}

impl From<ParseIntError> for Error {
    fn from(inner: ParseIntError) -> Self {
        Error::ParseIntError(inner)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(value: &'a str) -> Self {
        Self::Message(value.to_owned())
    }
}
