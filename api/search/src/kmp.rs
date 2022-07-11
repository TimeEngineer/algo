fn kmp_table<T: PartialEq>(pattern: &[T], table: &mut [usize]) {
    for i in 1..pattern.len() {
        let mut j = table[i - 1];
        while j > 0 && pattern[j] != pattern[i] {
            j = table[j - 1];
        }
        table[i] = if pattern[j] == pattern[i] { j + 1 } else { j };
    }
}

pub fn kmp<T: PartialEq>(pattern: &[T], txt: &[T]) -> isize {
    if pattern.is_empty() || txt.is_empty() || pattern.len() > txt.len() {
        return -1;
    }

    let mut table = vec![0; pattern.len()];
    kmp_table(pattern, &mut table);

    let mut j = 0;

    for (i, c) in txt.iter().enumerate() {
        while j > 0 && *c != pattern[j] {
            j = table[j - 1];
        }
        if *c == pattern[j] {
            j += 1;
        }
        if j == pattern.len() {
            return (i + 1 - j) as isize;
        }
    }
    -1
}

pub fn kmp_all<T: PartialEq>(pattern: &[T], txt: &[T]) -> Vec<usize> {
    if pattern.is_empty() || txt.is_empty() || pattern.len() > txt.len() {
        return vec![];
    }

    let mut out = vec![];
    let mut table = vec![0; pattern.len()];
    kmp_table(pattern, &mut table);

    let mut j = 0;

    for (i, c) in txt.iter().enumerate() {
        while j > 0 && *c != pattern[j] {
            j = table[j - 1];
        }
        if *c == pattern[j] {
            j += 1;
        }
        if j == pattern.len() {
            out.push(i + 1 - j);
            j = table[j - 1];
        }
    }
    out
}

#[test]
fn each_letter_matches() {
    assert_eq!(kmp(b"a", b"aaa"), 0);
    assert_eq!(kmp_all(b"a", b"aaa"), vec![0, 1, 2]);
}

#[test]
fn a_few_separate_matches() {
    assert_eq!(kmp(b"ab", b"abababa"), 0);
    assert_eq!(kmp_all(b"ab", b"abababa"), vec![0, 2, 4]);
}

#[test]
fn one_match() {
    assert_eq!(kmp(b"ABCDABD", b"ABC ABCDAB ABCDABCDABDE"), 15);
    assert_eq!(kmp_all(b"ABCDABD", b"ABC ABCDAB ABCDABCDABDE"), vec![15]);
}

#[test]
fn lots_of_matches() {
    assert_eq!(kmp(b"aa", b"aaabaabaaaaa"), 0);
    assert_eq!(kmp_all(b"aa", b"aaabaabaaaaa"), vec![0, 1, 4, 7, 8, 9, 10]);
}

#[test]
fn lots_of_intricate_matches() {
    assert_eq!(kmp(b"aba", b"ababababa"), 0);
    assert_eq!(kmp_all(b"aba", b"ababababa"), vec![0, 2, 4, 6]);
}

#[test]
fn not_found0() {
    assert_eq!(kmp(b"f", b"abcde"), -1);
    assert_eq!(kmp_all(b"f", b"abcde"), vec![]);
}

#[test]
fn not_found1() {
    assert_eq!(kmp(b"ac", b"abcde"), -1);
    assert_eq!(kmp_all(b"ac", b"abcde"), vec![]);
}

#[test]
fn not_found2() {
    assert_eq!(kmp(b"bababa", b"ababab"), -1);
    assert_eq!(kmp_all(b"bababa", b"ababab"), vec![]);
}

#[test]
fn empty_pattern() {
    assert_eq!(kmp(b"", b"abcdef"), -1);
    assert_eq!(kmp_all(b"", b"abcdef"), vec![]);
}

#[test]
fn empty_txt() {
    assert_eq!(kmp(b"abcdef", b""), -1);
    assert_eq!(kmp_all(b"abcdef", b""), vec![]);
}

#[test]
fn pattern_longer_than_txt() {
    assert_eq!(kmp(b"abcdef", b"a"), -1);
    assert_eq!(kmp_all(b"abcdef", b"a"), vec![]);
}
