use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

type DynError = Box<dyn Error>;
type DynResult<T> = Result<T, DynError>;

pub fn parse_input<T: FromStr>() -> DynResult<T>
where
    DynError: From<<T as FromStr>::Err>,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse::<T>()?)
}

pub fn split_input<T: FromStr>() -> DynResult<Vec<T>>
where
    DynError: From<<T as FromStr>::Err>,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn join<T: Display>(a: &[T], sep: &str) -> String {
    a.iter()
        .map(|ai| ai.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}

fn main() -> DynResult<()> {
    let t = parse_input()?;
    for _ in 0..t {
        solve()?;
    }
    Ok(())
}

// -------------------- Your code start here -------------------- //

fn solve() -> DynResult<()> {
    Ok(())
}
