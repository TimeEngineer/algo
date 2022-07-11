#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::cmp::{Ordering, PartialOrd};
use core::convert::TryFrom;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use rand::rngs::ThreadRng;
use rand::Rng;

#[cfg(test)]
use core::f32::consts::FRAC_1_SQRT_2;

#[cfg(test)]
const EPSILON: f32 = 1e-6;

// ----------------------------------------
// F32x4
// ----------------------------------------

#[derive(Clone, Copy)]
pub struct F32x4(pub __m128);

impl F32x4 {
    #[inline(always)]
    unsafe fn store(self, mem_addr: *mut __m128) {
        _mm_store_ps(mem_addr as _, self.0);
    }

    #[inline(always)]
    pub fn new(a0: f32, a1: f32, a2: f32, a3: f32) -> Self {
        Self(unsafe { _mm_set_ps(a3, a2, a1, a0) })
    }

    #[inline(always)]
    pub fn from_array(a: [f32; 4]) -> Self {
        Self(unsafe { _mm_set_ps(a[3], a[2], a[1], a[0]) })
    }

    #[inline(always)]
    pub fn rand(rng: &mut ThreadRng) -> Self {
        let mut a = [0.; 4];
        rng.fill(&mut a[..]);
        Self(unsafe { _mm_set_ps(a[3], a[2], a[1], a[0]) })
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self(unsafe { _mm_setzero_ps() })
    }

    #[inline(always)]
    pub fn broadcast_with(a: f32) -> Self {
        Self(unsafe { _mm_set1_ps(a) })
    }

    #[inline(always)]
    pub fn to_array(self) -> [f32; 4] {
        let v = &mut [Self::zero()];
        unsafe { self.store(v.as_mut_ptr() as _) };
        let t = unsafe { core::slice::from_raw_parts(v.as_ptr() as _, 4) };
        <[f32; 4]>::try_from(t).unwrap()
    }

    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self(unsafe { _mm_max_ps(self.0, other.0) })
    }

    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self(unsafe { _mm_min_ps(self.0, other.0) })
    }

    #[inline(always)]
    pub fn shuffle<const MASK: i32>(self, other: Self) -> Self {
        Self(unsafe { _mm_shuffle_ps::<MASK>(self.0, other.0) })
    }

    #[inline(always)]
    pub fn permute<const MASK: i32>(self) -> Self {
        Self(unsafe { _mm_permute_ps::<MASK>(self.0) })
    }

    #[inline(always)]
    pub fn splat0(self) -> Self {
        self.permute::<0b00_00_00_00>()
    }

    #[inline(always)]
    pub fn splat1(self) -> Self {
        self.permute::<0b01_01_01_01>()
    }

    #[inline(always)]
    pub fn splat2(self) -> Self {
        self.permute::<0b10_10_10_10>()
    }

    #[inline(always)]
    pub fn splat3(self) -> Self {
        self.permute::<0b11_11_11_11>()
    }

    #[inline(always)]
    pub fn sum(self) -> Self {
        let p0 = self.permute::<0b10_11_00_01>();
        let a0 = self + p0;
        let p1 = a0.permute::<0b01_00_11_10>();
        a0 + p1
    }

    #[inline(always)]
    pub fn pack(self, other0: Self, other1: Self, other2: Self) -> F32x4 {
        let s0 = self.shuffle::<0b11_10_01_00>(other0);
        let s1 = other1.shuffle::<0b11_10_01_00>(other2);
        s0.shuffle::<0b11_01_10_00>(s1)
    }

    #[inline(always)]
    pub fn rev_half(self) -> Self {
        Self(unsafe { _mm_rcp_ps(self.0) })
    }

    #[inline(always)]
    pub fn rev(self) -> Self {
        let two = Self::broadcast_with(2.);
        let x0 = self.rev_half();
        x0 * (two - self * x0)
    }

    #[inline(always)]
    pub fn rsqrt_half(self) -> Self {
        Self(unsafe { _mm_rsqrt_ps(self.0) })
    }

    #[inline(always)]
    pub fn rsqrt(self) -> Self {
        let half = Self::broadcast_with(0.5);
        let three = Self::broadcast_with(3.);
        let x0 = self.rsqrt_half();
        half * x0 * (three - self * x0 * x0)
    }
}

impl Add for F32x4 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self(unsafe { _mm_add_ps(self.0, other.0) })
    }
}

impl Sub for F32x4 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self(unsafe { _mm_sub_ps(self.0, other.0) })
    }
}

impl Mul for F32x4 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self(unsafe { _mm_mul_ps(self.0, other.0) })
    }
}

impl Div for F32x4 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: Self) -> Self {
        Self(unsafe { _mm_div_ps(self.0, other.0) })
    }
}

impl AddAssign for F32x4 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.0 = unsafe { _mm_add_ps(self.0, other.0) }
    }
}

impl SubAssign for F32x4 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.0 = unsafe { _mm_sub_ps(self.0, other.0) }
    }
}

impl MulAssign for F32x4 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.0 = unsafe { _mm_mul_ps(self.0, other.0) }
    }
}

impl DivAssign for F32x4 {
    #[inline(always)]
    fn div_assign(&mut self, other: Self) {
        self.0 = unsafe { _mm_div_ps(self.0, other.0) }
    }
}

impl PartialEq<F32x4> for F32x4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { _mm_comieq_ss(self.0, other.0) == 1 }
    }
}

impl PartialOrd for F32x4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(unsafe { (_mm_comigt_ss(self.0, other.0)).cmp(&0) })
    }
}

#[test]
fn align_f32x4() {
    assert_eq!(core::mem::align_of::<F32x4>(), 16);
}

#[test]
fn rand_f32x4() {
    let mut rng = rand::thread_rng();
    let _ = F32x4::rand(&mut rng);
}

#[test]
fn new_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    assert_eq!(a.to_array(), [0., 1., 2., 3.]);
}

#[test]
fn zero_f32x4() {
    let a = F32x4::zero();
    assert_eq!(a.to_array(), [0.; 4]);
}

#[test]
fn broadcast_with_f32x4() {
    let a = F32x4::broadcast_with(1.);
    assert_eq!(a.to_array(), [1.; 4]);
}

#[test]
fn add_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(0., 1., 2., 3.);
    let c = a + b;
    assert_eq!(c.to_array(), [0., 2., 4., 6.]);
}

#[test]
fn add_assign_f32x4() {
    let mut a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(0., 1., 2., 3.);
    a += b;
    assert_eq!(a.to_array(), [0., 2., 4., 6.]);
}

#[test]
fn sub_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(0., 1., 2., 3.);
    let c = a - b;
    assert_eq!(c.to_array(), [0., 0., 0., 0.]);
}

#[test]
fn sub_assign_f32x4() {
    let mut a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(0., 1., 2., 3.);
    a -= b;
    assert_eq!(a.to_array(), [0., 0., 0., 0.]);
}

#[test]
fn mul_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(0., 1., 2., 3.);
    let c = a * b;
    assert_eq!(c.to_array(), [0., 1., 4., 9.]);
}

#[test]
fn mul_assign_f32x4() {
    let mut a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(0., 1., 2., 3.);
    a *= b;
    assert_eq!(a.to_array(), [0., 1., 4., 9.]);
}

#[test]
fn div_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(1., 1., 2., 3.);
    let c = a / b;
    assert_eq!(c.to_array(), [0., 1., 1., 1.]);
}

#[test]
fn div_assign_f32x4() {
    let mut a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(1., 1., 2., 3.);
    a /= b;
    assert_eq!(a.to_array(), [0., 1., 1., 1.]);
}

#[test]
fn max_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(3., 2., 1., 0.);
    let c = a.max(b);
    assert_eq!(c.to_array(), [3., 2., 2., 3.]);
}

#[test]
fn min_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(3., 2., 1., 0.);
    let c = a.min(b);
    assert_eq!(c.to_array(), [0., 1., 1., 0.]);
}

#[test]
fn shuffle_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(4., 5., 6., 7.);
    let c = a.shuffle::<0b00_00_00_00>(b);
    assert_eq!(c.to_array(), [0., 0., 4., 4.]);

    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(4., 5., 6., 7.);
    let c = a.shuffle::<0b00_00_00_01>(b);
    assert_eq!(c.to_array(), [1., 0., 4., 4.]);

    let a = F32x4::new(0., 1., 2., 3.);
    let b = F32x4::new(4., 5., 6., 7.);
    let c = a.shuffle::<0b11_10_01_00>(b);
    assert_eq!(c.to_array(), [0., 1., 6., 7.]);
}

#[test]
fn permute_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = a.permute::<0b00_00_00_00>();
    assert_eq!(b.to_array(), [0., 0., 0., 0.]);

    let a = F32x4::new(0., 1., 2., 3.);
    let b = a.permute::<0b00_00_00_01>();
    assert_eq!(b.to_array(), [1., 0., 0., 0.]);

    let a = F32x4::new(0., 1., 2., 3.);
    let b = a.permute::<0b11_10_01_00>();
    assert_eq!(b.to_array(), [0., 1., 2., 3.]);
}

#[test]
fn splat_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = a.splat0();
    assert_eq!(b.to_array(), [0., 0., 0., 0.]);

    let b = a.splat1();
    assert_eq!(b.to_array(), [1., 1., 1., 1.]);

    let b = a.splat2();
    assert_eq!(b.to_array(), [2., 2., 2., 2.]);

    let b = a.splat3();
    assert_eq!(b.to_array(), [3., 3., 3., 3.]);
}

#[test]
fn sum_f32x4() {
    let a = F32x4::new(0., 1., 2., 3.);
    let b = a.sum();
    assert_eq!(b.to_array(), [6., 6., 6., 6.]);
}

#[test]
fn pack_f32x4() {
    let a = F32x4::new(0., 0., 0., 0.);
    let b = F32x4::new(1., 1., 1., 1.);
    let c = F32x4::new(2., 2., 2., 2.);
    let d = F32x4::new(3., 3., 3., 3.);
    let e = a.pack(b, c, d);
    assert_eq!(e.to_array(), [0., 1., 2., 3.]);
}

#[test]
fn rev_f32x4() {
    let a = F32x4::new(1., 2., 4., 8.);
    let b = a.rev();
    let c = F32x4::new(1., 0.5, 0.25, 0.125);
    for x in (b - c).to_array().iter() {
        assert!(x.abs() < EPSILON);
    }
}

#[test]
fn rsqrt_f32x4() {
    let a = F32x4::new(1., 4., 16., 2.);
    let b = a.rsqrt();
    let c = F32x4::new(1., 0.5, 0.25, FRAC_1_SQRT_2);
    for x in (b - c).to_array().iter() {
        assert!(x.abs() < EPSILON);
    }
}

// ----------------------------------------
// F32x8
// ----------------------------------------

#[derive(Clone, Copy)]
pub struct F32x8(pub __m256);

impl F32x8 {
    #[inline(always)]
    unsafe fn store(self, mem_addr: *mut __m256) {
        _mm256_store_ps(mem_addr as _, self.0);
    }

    #[inline(always)]
    pub fn new(a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32) -> Self {
        Self(unsafe { _mm256_set_ps(a7, a6, a5, a4, a3, a2, a1, a0) })
    }

    #[inline(always)]
    pub fn from_array(a: [f32; 8]) -> Self {
        Self(unsafe { _mm256_set_ps(a[7], a[6], a[5], a[4], a[3], a[2], a[1], a[0]) })
    }

    #[inline(always)]
    pub fn rand(rng: &mut ThreadRng) -> Self {
        let mut a = [0.; 8];
        rng.fill(&mut a[..]);
        Self(unsafe { _mm256_set_ps(a[7], a[6], a[5], a[4], a[3], a[2], a[1], a[0]) })
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self(unsafe { _mm256_setzero_ps() })
    }

    #[inline(always)]
    pub fn broadcast_with(a: f32) -> Self {
        Self(unsafe { _mm256_set1_ps(a) })
    }

    #[inline(always)]
    pub fn to_array(self) -> [f32; 8] {
        let v = &mut [Self::zero()];
        unsafe { self.store(v.as_mut_ptr() as _) };
        let t = unsafe { core::slice::from_raw_parts(v.as_ptr() as _, 8) };
        <[f32; 8]>::try_from(t).unwrap()
    }

    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self(unsafe { _mm256_max_ps(self.0, other.0) })
    }

    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self(unsafe { _mm256_min_ps(self.0, other.0) })
    }

    #[inline(always)]
    pub fn shuffle<const MASK: i32>(self, other: Self) -> Self {
        Self(unsafe { _mm256_shuffle_ps::<MASK>(self.0, other.0) })
    }

    #[inline(always)]
    pub fn permute<const MASK: i32>(self) -> Self {
        Self(unsafe { _mm256_permute_ps::<MASK>(self.0) })
    }

    #[inline(always)]
    pub fn splat0(self) -> Self {
        self.permute::<0b00_00_00_00>()
    }

    #[inline(always)]
    pub fn splat1(self) -> Self {
        self.permute::<0b01_01_01_01>()
    }

    #[inline(always)]
    pub fn splat2(self) -> Self {
        self.permute::<0b10_10_10_10>()
    }

    #[inline(always)]
    pub fn splat3(self) -> Self {
        self.permute::<0b11_11_11_11>()
    }

    #[inline(always)]
    pub fn rev_half(self) -> Self {
        Self(unsafe { _mm256_rcp_ps(self.0) })
    }

    #[inline(always)]
    pub fn rev(self) -> Self {
        let two = Self::broadcast_with(2.);
        let x0 = self.rev_half();
        x0 * (two - self * x0)
    }

    #[inline(always)]
    pub fn rsqrt_half(self) -> Self {
        Self(unsafe { _mm256_rsqrt_ps(self.0) })
    }

    #[inline(always)]
    pub fn rsqrt(self) -> Self {
        let half = Self::broadcast_with(0.5);
        let three = Self::broadcast_with(3.);
        let x0 = self.rsqrt_half();
        half * x0 * (three - self * x0 * x0)
    }

    #[inline(always)]
    pub fn madd(self, a: Self, b: Self) -> Self {
        Self(unsafe { _mm256_fmadd_ps(self.0, a.0, b.0) })
    }
}

impl Add for F32x8 {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self(unsafe { _mm256_add_ps(self.0, other.0) })
    }
}

impl Sub for F32x8 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self(unsafe { _mm256_sub_ps(self.0, other.0) })
    }
}

impl Mul for F32x8 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self(unsafe { _mm256_mul_ps(self.0, other.0) })
    }
}

impl Div for F32x8 {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: Self) -> Self {
        Self(unsafe { _mm256_div_ps(self.0, other.0) })
    }
}

impl AddAssign for F32x8 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.0 = unsafe { _mm256_add_ps(self.0, other.0) }
    }
}

impl SubAssign for F32x8 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.0 = unsafe { _mm256_sub_ps(self.0, other.0) }
    }
}

impl MulAssign for F32x8 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.0 = unsafe { _mm256_mul_ps(self.0, other.0) }
    }
}

impl DivAssign for F32x8 {
    #[inline(always)]
    fn div_assign(&mut self, other: Self) {
        self.0 = unsafe { _mm256_div_ps(self.0, other.0) }
    }
}

#[test]
fn align_f32x8() {
    assert_eq!(core::mem::align_of::<F32x8>(), 32);
}

#[test]
fn rand_f32x8() {
    let mut rng = rand::thread_rng();
    let _ = F32x8::rand(&mut rng);
}

#[test]
fn new_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    assert_eq!(a.to_array(), [0., 1., 2., 3., 4., 5., 6., 7.]);
}

#[test]
fn zero_f32x8() {
    let a = F32x8::zero();
    assert_eq!(a.to_array(), [0.; 8]);
}

#[test]
fn broadcast_with_f32x8() {
    let a = F32x8::broadcast_with(1.);
    assert_eq!(a.to_array(), [1.; 8]);
}

#[test]
fn add_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let c = a + b;
    assert_eq!(c.to_array(), [0., 2., 4., 6., 8., 10., 12., 14.]);
}

#[test]
fn add_assign_f32x8() {
    let mut a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    a += b;
    assert_eq!(a.to_array(), [0., 2., 4., 6., 8., 10., 12., 14.]);
}

#[test]
fn sub_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let c = a - b;
    assert_eq!(c.to_array(), [0., 0., 0., 0., 0., 0., 0., 0.]);
}

#[test]
fn sub_assign_f32x8() {
    let mut a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    a -= b;
    assert_eq!(a.to_array(), [0., 0., 0., 0., 0., 0., 0., 0.]);
}

#[test]
fn mul_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let c = a * b;
    assert_eq!(c.to_array(), [0., 1., 4., 9., 16., 25., 36., 49.]);
}

#[test]
fn mul_assign_f32x8() {
    let mut a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    a *= b;
    assert_eq!(a.to_array(), [0., 1., 4., 9., 16., 25., 36., 49.]);
}

#[test]
fn div_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(1., 1., 2., 3., 4., 5., 6., 7.);
    let c = a / b;
    assert_eq!(c.to_array(), [0., 1., 1., 1., 1., 1., 1., 1.]);
}

#[test]
fn div_assign_f32x8() {
    let mut a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(1., 1., 2., 3., 4., 5., 6., 7.);
    a /= b;
    assert_eq!(a.to_array(), [0., 1., 1., 1., 1., 1., 1., 1.]);
}

#[test]
fn max_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(7., 6., 5., 4., 3., 2., 1., 0.);
    let c = a.max(b);
    assert_eq!(c.to_array(), [7., 6., 5., 4., 4., 5., 6., 7.]);
}

#[test]
fn min_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(7., 6., 5., 4., 3., 2., 1., 0.);
    let c = a.min(b);
    assert_eq!(c.to_array(), [0., 1., 2., 3., 3., 2., 1., 0.]);
}

#[test]
fn shuffle_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(8., 9., 10., 11., 12., 13., 14., 15.);
    let c = a.shuffle::<0b00_00_00_00>(b);
    assert_eq!(c.to_array(), [0., 0., 8., 8., 4., 4., 12., 12.]);

    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(8., 9., 10., 11., 12., 13., 14., 15.);
    let c = a.shuffle::<0b00_00_00_01>(b);
    assert_eq!(c.to_array(), [1., 0., 8., 8., 5., 4., 12., 12.]);

    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(8., 9., 10., 11., 12., 13., 14., 15.);
    let c = a.shuffle::<0b11_10_01_00>(b);
    assert_eq!(c.to_array(), [0., 1., 10., 11., 4., 5., 14., 15.]);
}

#[test]
fn permute_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = a.permute::<0b00_00_00_00>();
    assert_eq!(b.to_array(), [0., 0., 0., 0., 4., 4., 4., 4.]);

    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = a.permute::<0b00_00_00_01>();
    assert_eq!(b.to_array(), [1., 0., 0., 0., 5., 4., 4., 4.]);

    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = a.permute::<0b11_10_01_00>();
    assert_eq!(b.to_array(), [0., 1., 2., 3., 4., 5., 6., 7.]);
}

#[test]
fn splat_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = a.splat0();
    assert_eq!(b.to_array(), [0., 0., 0., 0., 4., 4., 4., 4.]);

    let b = a.splat1();
    assert_eq!(b.to_array(), [1., 1., 1., 1., 5., 5., 5., 5.]);

    let b = a.splat2();
    assert_eq!(b.to_array(), [2., 2., 2., 2., 6., 6., 6., 6.]);

    let b = a.splat3();
    assert_eq!(b.to_array(), [3., 3., 3., 3., 7., 7., 7., 7.]);
}

#[test]
fn rev_f32x8() {
    let a = F32x8::new(1., 2., 4., 8., 1., 1., 1., 1.);
    let b = a.rev();
    let c = F32x8::new(1., 0.5, 0.25, 0.125, 1., 1., 1., 1.);
    for x in (b - c).to_array().iter() {
        assert!(x.abs() < EPSILON);
    }
}

#[test]
fn rsqrt_f32x8() {
    let a = F32x8::new(1., 4., 16., 2., 1., 1., 1., 1.);
    let b = a.rsqrt();
    let c = F32x8::new(1., 0.5, 0.25, FRAC_1_SQRT_2, 1., 1., 1., 1.);
    for x in (b - c).to_array().iter() {
        assert!(x.abs() < EPSILON);
    }
}

#[test]
fn madd_f32x8() {
    let a = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let b = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let c = F32x8::new(0., 1., 2., 3., 4., 5., 6., 7.);
    let d = a.madd(b, c);
    assert_eq!(d.to_array(), [0., 2., 6., 12., 20., 30., 42., 56.]);
}
