fn solve() -> DynResult<()> {
    let outs = p(3);
    for out in outs {
        println!("{}", out);
        let n = parse_input::<usize>()?;

        if n == 0 {
            break;
        }
    }
    Ok(())
}

fn appendzero(s: &str) -> String {
    s.to_owned() + &join(&vec![0; s.len()], "")
}

fn expand(s: &str) -> String {
    s.to_owned() + s
}

fn p(k: usize) -> Vec<String> {
    if k == 0 {
        return vec!["1".to_string()];
    }
    let seq = p(k - 1);
    let seq_with_zero: Vec<_> = seq.iter().map(|s| appendzero(s)).collect();
    let seq_with_copy: Vec<_> = seq.iter().map(|s| expand(s)).collect();
    let mut res = seq_with_copy.clone();
    for ins in seq_with_zero {
        res.push(ins);
        res.append(&mut seq_with_copy.clone())
    }
    res
}