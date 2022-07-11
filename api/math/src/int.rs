use core::cmp::Ordering;
use core::fmt::Display;
use core::ops::{Add, BitXor, Div, Mul, Neg, Not, Rem, Sub};
use std::error::Error;
use std::str::FromStr;

const BASE: u64 = 10000000000000000000;
const POWER: usize = 19;
const RADIX: u64 = 10;

fn cmp(a: &[u64], b: &[u64]) -> Ordering {
    if a.len() > b.len() {
        Ordering::Greater
    } else if a.len() < b.len() {
        Ordering::Less
    } else if a.is_empty() {
        Ordering::Equal
    } else {
        let mut size = a.len() - 1;
        let mut order = a[size].cmp(&b[size]);
        while order == Ordering::Equal {
            if size == 0 {
                return Ordering::Equal;
            }
            size -= 1;
            order = a[size].cmp(&b[size]);
        }
        order
    }
}

fn clear(a: &mut [u64]) {
    for ai in a {
        *ai = 0
    }
}

fn len(a: &[u64]) -> usize {
    match a.iter().rposition(|&x| x != 0) {
        Some(len) => len + 1,
        None => 0,
    }
}

fn shift_left(a: &mut [u64], shift: usize) {
    if shift > 0 {
        let mut carry: u64 = 0;
        let mut new_carry: u64;
        for elem in a.iter_mut() {
            new_carry = *elem >> (64 - shift);
            *elem = (*elem << shift) | carry;
            carry = new_carry;
        }
    }
}

fn shift_right(a: &mut [u64], shift: usize) {
    if shift > 0 {
        let mut borrow: u64 = 0;
        let mut new_borrow: u64;
        for elem in a.iter_mut().rev() {
            new_borrow = *elem << (64 - shift);
            *elem = (*elem >> shift) | borrow;
            borrow = new_borrow;
        }
    }
}

fn add_kernel(a: u64, b: u64, carry: &mut u128) -> u64 {
    *carry += a as u128;
    *carry += b as u128;
    let out = *carry as u64;
    *carry >>= 64;
    out
}

// a > b
fn add0(a: &[u64], b: &[u64], out: &mut [u64]) {
    let mut carry = 0;
    let (a_lo, a_hi) = a.split_at(b.len());
    let (out_lo, out_hi) = out.split_at_mut(b.len());
    for (ai, bi, outi) in a_lo
        .iter()
        .zip(b.iter())
        .zip(out_lo.iter_mut())
        .map(|((a, b), c)| (a, b, c))
    {
        *outi = add_kernel(*ai, *bi, &mut carry)
    }
    for (ai, outi) in a_hi.iter().zip(out_hi) {
        *outi = add_kernel(*ai, 0, &mut carry)
    }
    if carry != 0 {
        out[out.len() - 1] = 1
    }
}

// a > b
fn add1(a: &mut [u64], b: &[u64]) {
    let mut carry = 0;
    let (a_lo, a_hi) = a.split_at_mut(b.len());
    for (ai, bi) in a_lo.iter_mut().zip(b) {
        *ai = add_kernel(*ai, *bi, &mut carry)
    }
    if carry != 0 {
        for ai in a_hi {
            *ai = add_kernel(*ai, 0, &mut carry);
            if carry == 0 {
                break;
            }
        }
    }
}

fn sub_kernel(a: u64, b: u64, borrow: &mut i128) -> u64 {
    *borrow += a as i128;
    *borrow -= b as i128;
    let out = *borrow as u64;
    *borrow >>= 64;
    out
}

// a > b
fn sub0(a: &[u64], b: &[u64], out: &mut [u64]) {
    let mut borrow = 0;
    let (a_lo, a_hi) = a.split_at(b.len());
    let (out_lo, out_hi) = out.split_at_mut(b.len());
    for (ai, bi, outi) in a_lo
        .iter()
        .zip(b.iter())
        .zip(out_lo.iter_mut())
        .map(|((a, b), c)| (a, b, c))
    {
        *outi = sub_kernel(*ai, *bi, &mut borrow)
    }
    for (ai, outi) in a_hi.iter().zip(out_hi) {
        *outi = sub_kernel(*ai, 0, &mut borrow)
    }
}

// a > b
fn sub1(a: &mut [u64], b: &[u64]) {
    let mut borrow = 0;
    if cmp(a, b) == Ordering::Less {
        for a in a.iter_mut() {
            *a = 0
        }
    }
    // wrapping
    else {
        let (a_lo, a_hi) = a.split_at_mut(b.len());
        for (ai, bi) in a_lo.iter_mut().zip(b) {
            *ai = sub_kernel(*ai, *bi, &mut borrow)
        }
        for ai in a_hi {
            *ai = sub_kernel(*ai, 0, &mut borrow)
        }
    }
}

fn mul_kernel(a: u64, b: u64, c: u64, carry: &mut u128) -> u64 {
    *carry += a as u128;
    *carry += (b as u128) * (c as u128);
    let out = *carry as u64;
    *carry >>= 64;
    out
}

fn mul0(a: &[u64], b: u64, out: &mut [u64]) {
    if b == 0 {
        return;
    }
    let mut carry = 0;
    let (out_lo, out_hi) = out.split_at_mut(a.len());
    for (ai, outi) in a.iter().zip(out_lo) {
        *outi = mul_kernel(*outi, *ai, b, &mut carry)
    }
    if carry != 0 {
        for outi in out_hi {
            *outi = add_kernel(*outi, 0, &mut carry);
            if carry == 0 {
                break;
            }
        }
    }
}

// a > b
fn mul_long(a: &[u64], b: &[u64], out: &mut [u64]) {
    for (i, bi) in b.iter().enumerate() {
        mul0(a, *bi, &mut out[i..])
    }
}

// a > b
fn mul_karatsuba(a: &[u64], b: &[u64], out: &mut [u64], temp: &mut [u64]) {
    let len = b.len() >> 1;
    let (b0, b1) = b.split_at(len);
    let (a0, a1) = a.split_at(len);
    // a = (a1, a0), b = (b1, b0)

    // out = p2 * x2 + (p1 - p2 - p0) * x + p0;
    let i = b.len() + 1 - len;
    let j = i + a.len() + 1 - len;
    add0(b1, b0, &mut temp[..i]);
    add0(a1, a0, &mut temp[i..j]);

    // p1 = (a0 + a1) * (b0 + b1)
    mul(&temp[i..j], &temp[..i], &mut out[len..]);

    let i = len << 1;
    let j = j - 2;
    // p2 = a1 * b1
    clear(&mut temp[..j]);
    mul(a1, b1, &mut temp[..j]);
    add1(&mut out[i..], &temp[..j]);
    sub1(&mut out[len..], &temp[..j]);

    // p0 = a0 * b0
    clear(&mut temp[..i]);
    mul(a0, b0, &mut temp[..i]);
    sub1(&mut out[len..], &temp[..i]);
    add1(out, &temp[..i]);
}

fn mul(a: &[u64], b: &[u64], out: &mut [u64]) {
    if b.is_empty() {
        return;
    }

    if b.len() < 16 {
        mul_long(a, b, out);
    } else {
        let mut temp = vec![0; a.len() + 3];
        mul_karatsuba(a, b, out, &mut temp[..]);
    }
}

fn div_rem_kernel(hi: u64, lo: u64, divisor: u64) -> (u64, u64) {
    let a = ((hi as u128) << 64) | (lo as u128);
    let b = divisor as u128;
    ((a / b) as u64, (a % b) as u64)
}

fn div_rem0(a: &mut [u64], b: u64) -> u64 {
    let mut rem = 0;
    for ai in a.iter_mut().rev() {
        let (q, r) = div_rem_kernel(rem, *ai, b);
        *ai = q;
        rem = r;
    }
    rem
}

fn div_rem(a: &mut [u64], b: &mut [u64], out: &mut [u64], temp: &mut [u64]) {
    let shift = b.last().unwrap().leading_zeros() as usize;
    shift_left(b, shift);
    shift_left(a, shift);
    let bn = *b.last().unwrap();
    let mut a_len = len(a);

    for j in (0..out.len()).rev() {
        let offset = j + b.len() - 1;
        if offset >= a_len {
            continue;
        }

        let (temp0, temp1) = temp.split_at_mut(a_len - offset);

        temp0.copy_from_slice(&a[offset..a_len]);
        div_rem0(temp0, bn);

        let temp0_len = len(temp0);
        let temp1_len = b.len() + temp0_len;

        clear(&mut temp1[..temp1_len]);
        mul(b, &temp0[..temp0_len], &mut temp1[..temp1_len]);
        let temp1_len = len(&temp1[..temp1_len]);

        while cmp(&temp1[..temp1_len], &a[j..a_len]) == Ordering::Greater {
            sub1(&mut temp0[..temp0_len], &[1]);
            sub1(&mut temp1[..temp1_len], b);
        }

        add1(&mut out[j..], &temp0[..temp0_len]);
        sub1(&mut a[j..a_len], &temp1[..temp1_len]);
        a_len = len(&a[..a_len]);
    }
    shift_right(&mut a[..a_len], shift);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Int {
    pub value: Vec<u64>,
    pub sign: Sign,
}

impl Int {
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn zero() -> Self {
        Int {
            value: vec![],
            sign: Sign::Positive,
        }
    }
    pub fn one() -> Self {
        Int {
            value: vec![1],
            sign: Sign::Positive,
        }
    }
    pub fn from(value: Vec<u64>, sign: Sign) -> Self {
        Int { value, sign }
    }
    pub fn is_even(&self) -> bool {
        self.value[0] & 1 == 0
    }
    pub fn is_odd(&self) -> bool {
        self.value[0] & 1 == 1
    }
    pub fn is_zero(&self) -> bool {
        self.is_empty()
    }
    pub fn is_one(&self) -> bool {
        self.len() == 1 && self.value[0] == 1
    }
    pub fn is_positive(&self) -> bool {
        self.sign == Sign::Positive || self.is_zero()
    }
    pub fn is_negative(&self) -> bool {
        self.sign == Sign::Negative || self.is_zero()
    }
    pub fn abs(&self) -> Self {
        Int {
            value: self.value.clone(),
            sign: Sign::Positive,
        }
    }
    pub fn simplify(mut self) -> Self {
        while let Some(&0) = self.value.last() {
            self.value.pop();
        }
        self
    }
    pub fn exp(&self, n: usize) -> Self {
        if n == 1 {
            self.clone()
        } else if n & 1 == 0 {
            let square = self * self;
            square.exp(n >> 1)
        } else {
            let square = self * self;
            self * &square.exp(n >> 1)
        }
    }
    pub fn fact(&self) -> Self {
        if self.is_one() {
            self.clone()
        } else {
            let next = self - &Int::one();
            self * &next.fact()
        }
    }
}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Int {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp(&self.value[..], &other.value[..])
    }
}

impl Not for Sign {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

impl BitXor for Sign {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        match (self, other) {
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Negative,
            (Sign::Negative, Sign::Negative) => Sign::Positive,
        }
    }
}

impl<'a> Neg for &'a Int {
    type Output = Int;
    fn neg(self) -> Int {
        Int::from(self.value.clone(), !self.sign)
    }
}

impl<'a> Add for &'a Int {
    type Output = Int;
    fn add(self, other: Self) -> Int {
        if self.is_zero() {
            other.clone()
        } else if other.is_zero() {
            self.clone()
        } else if self.sign == other.sign {
            let mut out = vec![0; self.len() + 1];
            match self.len().cmp(&other.len()) {
                Ordering::Greater => {
                    add0(&self.value[..], &other.value[..], &mut out[..]);
                }
                _ => {
                    add0(&other.value[..], &self.value[..], &mut out[..]);
                }
            }
            Int::from(out, self.sign).simplify()
        } else {
            let mut out = vec![0; self.len()];
            match self.cmp(other) {
                Ordering::Greater => {
                    sub0(&self.value[..], &other.value[..], &mut out[..]);
                    Int::from(out, self.sign).simplify()
                }
                Ordering::Less => {
                    sub0(&other.value[..], &self.value[..], &mut out[..]);
                    Int::from(out, other.sign).simplify()
                }
                Ordering::Equal => Int::zero(),
            }
        }
    }
}

impl<'a> Sub for &'a Int {
    type Output = Int;
    fn sub(self, other: Self) -> Int {
        if self.is_zero() {
            -other
        } else if other.is_zero() {
            self.clone()
        } else if self.sign != other.sign {
            let mut out = vec![0; self.len() + 1];
            match self.len().cmp(&other.len()) {
                Ordering::Greater => {
                    add0(&self.value[..], &other.value[..], &mut out[..]);
                }
                _ => {
                    add0(&other.value[..], &self.value[..], &mut out[..]);
                }
            }
            Int::from(out, self.sign).simplify()
        } else {
            let mut out = vec![0; self.len()];
            match self.cmp(other) {
                Ordering::Greater => {
                    sub0(&self.value[..], &other.value[..], &mut out[..]);
                    Int::from(out, self.sign).simplify()
                }
                Ordering::Less => {
                    sub0(&other.value[..], &self.value[..], &mut out[..]);
                    Int::from(out, !self.sign).simplify()
                }
                Ordering::Equal => Int::zero(),
            }
        }
    }
}

impl<'a> Mul for &'a Int {
    type Output = Int;
    fn mul(self, other: Self) -> Int {
        if self.is_zero() || other.is_zero() {
            Int::zero()
        } else if self.is_one() {
            Int::from(other.value.clone(), self.sign ^ other.sign)
        } else if other.is_one() {
            Int::from(self.value.clone(), self.sign ^ other.sign)
        } else {
            let mut out = vec![0; self.len() + other.len()];
            match self.len().cmp(&other.len()) {
                Ordering::Greater => {
                    mul(&self.value[..], &other.value[..], &mut out[..]);
                }
                _ => {
                    mul(&other.value[..], &self.value[..], &mut out[..]);
                }
            }
            Int::from(out, self.sign ^ other.sign).simplify()
        }
    }
}

impl<'a> Div for &'a Int {
    type Output = Int;
    fn div(self, other: Self) -> Int {
        match self.cmp(other) {
            Ordering::Less => return Int::zero(),
            Ordering::Equal => return Int::from(vec![1], self.sign ^ other.sign),
            _ => {}
        }
        if other.is_one() {
            Int::from(self.value.clone(), self.sign ^ other.sign)
        } else {
            let mut r = self.value.clone();
            r.push(0);
            let mut d = other.value.clone();
            let mut q = vec![0; self.len() - other.len() + 1];
            let mut temp = vec![0; (r.len() << 1) - d.len() + 2];
            div_rem(&mut r[..], &mut d[..], &mut q[..], &mut temp[..]);
            Int::from(q, self.sign ^ other.sign).simplify()
        }
    }
}

impl<'a> Rem for &'a Int {
    type Output = Int;

    #[inline]
    fn rem(self, other: Self) -> Int {
        if self.cmp(other) == Ordering::Equal {
            return Int::zero();
        }
        if other.is_one() {
            Int::zero()
        } else {
            let mut r = self.value.clone();
            r.push(0);
            let mut d = other.value.clone();
            let mut q = vec![0; self.len() - other.len() + 1];
            let mut temp = vec![0; (r.len() << 1) - d.len() + 2];
            div_rem(&mut r[..], &mut d[..], &mut q[..], &mut temp[..]);
            Int::from(r, self.sign ^ other.sign).simplify()
        }
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.is_zero() {
            f.pad_integral(true, "", "0")
        } else {
            let mut digits = self.value.clone();
            let mut digits_len = digits.len();
            let bits = (digits_len << 6) - digits.last().unwrap().leading_zeros() as usize;
            let out_len = ((bits as f64) / (RADIX as f64).log2()).ceil() as usize;
            let mut out = Vec::with_capacity(out_len);
            while digits_len > 1 {
                let mut r = div_rem0(&mut digits[..digits_len], BASE);
                digits_len = len(&digits[..digits_len]);
                for _ in 0..POWER {
                    out.push((r % RADIX) as u8);
                    r /= RADIX;
                }
            }
            let mut r = digits[0];
            while r != 0 {
                out.push((r % RADIX) as u8);
                r /= RADIX;
            }
            for r in &mut out {
                *r += b'0';
            }
            out.reverse();
            f.pad_integral(
                self.sign == Sign::Positive,
                "",
                &String::from_utf8(out).unwrap(),
            )
        }
    }
}

impl FromStr for Int {
    type Err = IntError;
    fn from_str(s: &str) -> Result<Int, IntError> {
        if s.is_empty() {
            Err(IntError::EmptyValue)
        } else {
            let mut v = Vec::with_capacity(s.len());
            for b in s.bytes() {
                let d = match b {
                    b'0'..=b'9' => b - b'0',
                    b'_' | b' ' => continue,
                    _ => u8::max_value(),
                };
                if d < RADIX as u8 {
                    v.push(d)
                } else {
                    return Err(IntError::InvalidValue);
                }
            }
            let bits = (RADIX as f64).log2() * v.len() as f64;
            let out_len = (bits / 64.).ceil() as usize;
            let mut out = Vec::with_capacity(out_len);

            let r = v.len() % POWER;
            let i = if r == 0 { POWER } else { r };
            let (head, tail) = v.split_at(i);
            let first = head.iter().fold(0, |acc, &d| acc * RADIX + d as u64);
            out.push(first);
            for chunk in tail.chunks(POWER) {
                if out.last() != Some(&0) {
                    out.push(0)
                }
                let mut carry = 0;
                for d in out.iter_mut() {
                    *d = mul_kernel(0, *d, BASE, &mut carry)
                }
                let n = chunk.iter().fold(0, |acc, &d| acc * RADIX + d as u64);
                add1(&mut out, &[n]);
            }
            Ok(Int::from(out, Sign::Positive).simplify())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IntError {
    EmptyValue,
    InvalidValue,
}

impl IntError {
    fn description(&self) -> &str {
        match self {
            IntError::EmptyValue => "Error : Empty value",
            IntError::InvalidValue => "Error : Invalid value",
        }
    }
}

impl core::fmt::Display for IntError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for IntError {
    fn description(&self) -> &str {
        self.description()
    }
}
