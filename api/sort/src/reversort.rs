pub fn reversort<T: Ord>(a: &mut [T]) {
    for i in 0..a.len() - 1 {
        let j = a
            .iter()
            .enumerate()
            .skip(i)
            .min_by(|a, b| a.1.cmp(b.1))
            .map(|a| a.0)
            .unwrap()
            + 1;
        a[i..j].reverse();
    }
}
