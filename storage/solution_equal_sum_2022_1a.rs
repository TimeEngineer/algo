fn solve() -> DynResult<()> {
    let n = parse_input::<usize>()?;
    let mut set0 = Vec::with_capacity(n);
    let mut sum0 = 0;
    let mut sum1 = 0;

    // Phase 1
    for i in 7..30 {
        set0.push(1 << i);
    }
    for i in 1..n - 23 + 1 {
        set0.push(i);
    }
    println!("{}", join(&set0, " "));
    // Phase 2
    let mut set1 = split_input::<usize>()?;
    // Phase 3
    set0.append(&mut set1);
    set0.sort();
    set0.reverse();

    for value in set0 {
        if sum0 < sum1 {
            sum0 += value;
        } else {
            sum1 += value;
            set1.push(value);
        }
    }
    println!("{}", join(&set1, " "));
    Ok(())
}
