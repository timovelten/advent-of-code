use std::{io, num::ParseIntError};

mod day01;

fn main() -> crate::Result<()> {
    let solvers = [
        day01::solve    
    ];

    for (i, solver) in solvers.into_iter().enumerate() {
        let day = i + 1;
        println!("Day {}", day);

        let now = std::time::Instant::now();
        let _ = solver()?;
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