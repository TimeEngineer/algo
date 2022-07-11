use core::ops::*;

pub fn gcd<T: Default + Copy + PartialEq + Rem<Output = T>>(mut a: T, mut b: T) -> T {
    let zero = T::default();
    while b != zero {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
