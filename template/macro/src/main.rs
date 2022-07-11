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

use std::collections::HashMap;

fn main() {
    mem! {
        fn fib(n: usize, @x: usize) -> usize {
            // assert_eq!(x, 0);
            if n < 2 {
                return 1;
            } else {
                @fib(n - 1, x) + @fib(n - 2, x)
            }
        }
    }

    let mut mem = HashMap::new();
    println!("{}", fib(24, 0, &mut mem));
}
