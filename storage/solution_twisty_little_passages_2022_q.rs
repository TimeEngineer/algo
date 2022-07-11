use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn solve() -> DynResult<()> {
    let input = split_input::<usize>()?;
    let (n, k) = (input[0], input[1]);
    let input = split_input::<usize>()?;
    let (_, _) = (input[0], input[1]);
    let mut sum = 0;
    if k >= n {
        for i in 0..n {
            println!("T {}", i + 1);
            let input = split_input::<usize>()?;
            let (_, p) = (input[0], input[1]);
            sum += p;
        }
    } else {
        let mut map = HashMap::new();
        let mut hs = DefaultHasher::new();

        for i in 0..k / 2 {
            // Need rand from Google Code Jam
            (i + sum).hash(&mut hs);
            let rand = hs.finish() as usize % n;
            println!("T {}", rand + 1);
            let input = split_input::<usize>()?;
            let (r, p) = (input[0], input[1]);
            sum += p;
            map.insert(r, p);
            println!("W");
            let input = split_input::<usize>()?;
            let (r, p) = (input[0], input[1]);
            map.insert(r, p);
        }
        let deg = sum / (k / 2);
        for i in 1..=n {
            match map.get(&i) {
                Some(p) => sum += *p,
                None => sum += deg,
            }
        }
    }
    println!("E {}", sum / 2);
    Ok(())
}