pub fn de_bruijn(k: u8, n: usize) -> String {
    let alphabet = ('0'..(k + b'0') as char).collect::<Vec<_>>();
    let mut a = vec![0; k as usize * n];
    let mut sequence = vec![];

    fn db(a: &mut [usize], sequence: &mut Vec<usize>, t: usize, p: usize, k: usize, n: usize) {
        if t > n {
            if n % p == 0 {
                sequence.extend(&a[1..p + 1]);
            }
        } else {
            a[t] = a[t - p];
            db(a, sequence, t + 1, p, k, n);
            for j in a[t - p] + 1..k {
                a[t] = j;
                db(a, sequence, t + 1, t, k, n);
            }
        }
    }

    db(&mut a, &mut sequence, 1, 1, k as usize, n);
    sequence
        .into_iter()
        .map(|i| alphabet[i])
        .collect::<String>()
}

#[test]
fn de_bruijn_2_3() {
    assert_eq!(&de_bruijn(2, 3), "00010111")
}

#[test]
fn de_bruijn_2_4() {
    assert_eq!(&de_bruijn(2, 4), "0000100110101111")
}

#[test]
fn de_bruijn_3_2() {
    assert_eq!(&de_bruijn(3, 2), "001021122")
}
