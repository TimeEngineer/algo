use core::cmp::Ordering;

pub fn binary_search<T: Ord>(a: &[T], v: &T) -> isize {
    let mut low = 0;
    let mut high = a.len();

    while high > low {
        let half = (low + high) / 2;
        match v.cmp(&a[half]) {
            Ordering::Equal => return half as isize,
            Ordering::Greater => low = half + 1,
            Ordering::Less => high = half,
        }
    }
    -1
}

#[test]
fn length_is_even() {
    let a = [0, 1, 2, 3, 4, 5];
    assert_eq!(binary_search(&a, &5), 5);
}

#[test]
fn length_is_odd() {
    let a = [0, 1, 2, 3, 4];
    assert_eq!(binary_search(&a, &0), 0);
}

#[test]
fn empty() {
    let a = [];
    assert_eq!(binary_search(&a, &0), -1);
}

#[test]
fn middle() {
    let a = [0, 1, 2, 3, 4];
    assert_eq!(binary_search(&a, &2), 2);
}

#[test]
fn greater() {
    let a = [1, 2, 3, 4];
    assert_eq!(binary_search(&a, &5), -1);
}

#[test]
fn lower() {
    let a = [1, 2, 3, 4];
    assert_eq!(binary_search(&a, &0), -1);
}

#[test]
fn not_found() {
    let a = [1, 2, 4, 5];
    assert_eq!(binary_search(&a, &3), -1);
}
