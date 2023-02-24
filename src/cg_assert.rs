#![allow(clippy::no_effect)]
#![allow(clippy::erasing_op)]

/// Const functions that can be used to assert the const generics.

#[allow(dead_code)]
#[allow(path_statements)]
pub const fn assert_eq<const L: usize, const R: usize>() {
    Assert::<L, R>::EQ;
}

#[allow(dead_code)]
#[allow(path_statements)]
pub const fn assert_ne<const L: usize, const R: usize>() {
    Assert::<L, R>::NOT_EQ;
}

#[allow(dead_code)]
#[allow(path_statements)]
pub const fn assert_lt<const N: usize, const MAX: usize>() {
    Assert::<N, MAX>::LESS;
}

#[allow(dead_code)]
#[allow(path_statements)]
pub const fn assert_gt<const N: usize, const MIN: usize>() {
    Assert::<N, MIN>::GREATER;
}

#[allow(dead_code)]
#[allow(path_statements)]
pub const fn assert_lt_eq<const N: usize, const MAX: usize>() {
    Assert::<N, MAX>::LESS_EQ;
}

#[allow(dead_code)]
#[allow(path_statements)]
pub const fn assert_gt_eq<const N: usize, const MIN: usize>() {
    Assert::<N, MIN>::GREATER_EQ;
}

#[allow(dead_code)]
#[allow(path_statements)]
pub const fn assert_power_of_two<const N: usize>() {
    Assert::<N, 0>::GREATER;
    Assert::<N, 0>::POWER_OF_TWO;
}

#[allow(dead_code)]
/// Const assert hack
pub struct Assert<const L: usize, const R: usize>;

#[allow(dead_code)]
impl<const L: usize, const R: usize> Assert<L, R> {
    /// Const assert hack
    pub const EQ: usize = (R - L) + (L - R);
    /// Const assert hack
    pub const GREATER: usize = L - R - 1;
    /// Const assert hack
    pub const GREATER_EQ: usize = L - R;
    /// Const assert hack
    pub const LESS: usize = R - L - 1;
    /// Const assert hack
    pub const LESS_EQ: usize = R - L;
    /// Const assert hack
    pub const NOT_EQ: isize = 0 / (R as isize - L as isize);
    /// Const assert hack
    pub const POWER_OF_TWO: usize = 0 - (L & (L - 1));
}
