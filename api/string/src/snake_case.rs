pub fn to_snake_case(s: &str) -> String {
    let mut out = s[..1].to_lowercase();
    s.chars().skip(1).for_each(|c| match c {
        c @ 'A'..='Z' => {
            out.push('_');
            out.push((c as u8 - b'A' + b'a') as char)
        }
        c => out.push(c),
    });

    out
}
