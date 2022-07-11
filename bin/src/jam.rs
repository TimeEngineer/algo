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

#[allow(unused_macros)]
macro_rules! mem {
    // Entrypoint
    { fn $fun:ident ( $($arg:tt)* ) -> $out:ty { $($rest:tt)* } } => {
        mem! {
            meta = [$fun, $out]
            args = []
            cached = []
            rest = [$($arg)*]
            block = [$($rest)*]
        }
    };
    // Args
    {
        meta = [$fun:ident, $out:ty]
        args = [$($arg:tt)*]
        cached = [$($x:ident : $ty:ty,)*]
        rest = []
        block = [$($block:tt)*]
    } => {
        fn $fun( $($arg)* mem: &mut std::collections::HashMap<($($ty,)*), $out> ) -> $out {
            if let Some(v) = mem.get(&($($x,)*)) {
                v.clone()
            } else {
                let v = {
                    mem! {
                        meta = [mem]
                        block = []
                        rest = [$($block)*]
                    }
                };
                mem.insert(($($x,)*), v);
                v
            }
        }
    };
    // N variables not_cached
    {
        meta = [$fun:ident, $out:ty]
        args = [$($arg:tt)*]
        cached = [$($cached:tt)*]
        rest = [@$x:ident : $ty:ty, $($rest:tt)*]
        block = [$($block:tt)*]
    } => {
        mem! {
            meta = [$fun, $out]
            args = [$($arg)* $x : $ty,]
            cached = [$($cached)*]
            rest = [$($rest)*]
            block = [$($block)*]
        }
    };
    // 1 variable not_cached
    {
        meta = [$fun:ident, $out:ty]
        args = [$($arg:tt)*]
        cached = [$($cached:tt)*]
        rest = [@$x:ident : $ty:ty $(,)?]
        block = [$($block:tt)*]
    } => {
        mem! {
            meta = [$fun, $out]
            args = [$($arg)* $x : $ty,]
            cached = [$($cached)*]
            rest = []
            block = [$($block)*]
        }
    };
    // N variables
    {
        meta = [$fun:ident, $out:ty]
        args = [$($arg:tt)*]
        cached = [$($cached:tt)*]
        rest = [$x:ident : $ty:ty, $($rest:tt)*]
        block = [$($block:tt)*]
    } => {
        mem! {
            meta = [$fun, $out]
            args = [$($arg)* $x : $ty,]
            cached = [$($cached)* $x : $ty,]
            rest = [$($rest)*]
            block = [$($block)*]
        }
    };
    // 1 variable
    {
        meta = [$fun:ident, $out:ty]
        args = [$($arg:tt)*]
        cached = [$($cached:tt)*]
        rest = [$x:ident : $ty:ty $(,)?]
        block = [$($block:tt)*]
    } => {
        mem! {
            meta = [$fun, $out]
            args = [$($arg)* $x : $ty,]
            cached = [$($cached)* $x : $ty,]
            rest = []
            block = [$($block)*]
        }
    };
    // Block
    {
        meta = [$mem:ident]
        block = [$($block:tt)*]
        rest = []
    } => {
        $($block)*
    };
    // Fun
    {
        meta = [$mem:ident]
        block = [$($block:tt)*]
        rest = [@$fun:ident ($($arg:tt)*) $($rest:tt)*]
    } => {
        mem! {
            meta = [$mem]
            block = [$($block)* $fun($($arg)*, $mem)]
            rest = [$($rest)*]
        }
    };
    // Brace
    {
        meta = [$mem:ident]
        block = [$($block:tt)*]
        rest = [{ $($in:tt)* } $($rest:tt)*]
    } => {
        mem! {
            meta = [$mem]
            block = [$($block)* {
                mem! {
                    meta = [$mem]
                    block = []
                    rest = [$($in)*]
                }
            }]
            rest = [$($rest)*]
        }
    };
    // Token
    {
        meta = [$mem:ident]
        block = [$($block:tt)*]
        rest = [$next:tt $($rest:tt)*]
    } => {
        mem! {
            meta = [$mem]
            block = [$($block)* $next]
            rest = [$($rest)*]
        }
    };
    // Debug
    { $($t:tt)* } => {
        eprintln!("{}", stringify!($($t)*));
    }
}

fn main() -> DynResult<()> {
    let t = parse_input()?;
    for i in 0..t {
        println!("Case #{}: {}", i + 1, solve()?);
    }
    Ok(())
}

// -------------------- Your code start here -------------------- //

fn solve() -> DynResult<i32> {
    Ok(0)
}