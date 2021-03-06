//! Signed modulo 256-bit integer

use std::convert::{From, Into};
use std::ops::{Div, Rem};
use std::cmp::Ordering;

use super::{Sign, M256};
use super::u256::SIGN_BIT_MASK;
use super::algorithms::from_signed;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
/// Represents a signed module 256-bit integer.
pub struct MI256(Sign, M256);

impl MI256 {
    /// Zero value of MI256.
    pub fn zero() -> MI256 { MI256(Sign::NoSign, M256::zero()) }
    /// One value of MI256.
    pub fn one() -> MI256 { MI256(Sign::Plus, M256::one()) }
    /// Maximum value of MI256.
    pub fn max_value() -> MI256 { MI256(Sign::Plus, M256::max_value() & SIGN_BIT_MASK.into()) }
    /// Minimum value of MI256.
    pub fn min_value() -> MI256 { MI256(Sign::Minus, M256::min_value() & SIGN_BIT_MASK.into()) }
}

impl Default for MI256 { fn default() -> MI256 { MI256::zero() } }
impl From<M256> for MI256 {
    fn from(val: M256) -> MI256 {
        if val == M256::zero() {
            MI256::zero()
        } else if val & SIGN_BIT_MASK.into() == val {
            MI256(Sign::Plus, val)
        } else {
            let mut digits: [u32; 8] = val.into();
            from_signed(Sign::Minus, &mut digits);
            MI256(Sign::Minus, digits.into())
        }
    }
}
impl Into<M256> for MI256 {
    fn into(self) -> M256 {
        let sign = self.0;
        let mut digits: [u32; 8] = self.1.into();
        from_signed(sign, &mut digits);
        M256::from(digits)
    }
}

impl Ord for MI256 {
    fn cmp(&self, other: &MI256) -> Ordering {
        match (self.0, other.0) {
            (Sign::NoSign, Sign::NoSign) => Ordering::Equal,
            (Sign::NoSign, Sign::Plus) => Ordering::Less,
            (Sign::NoSign, Sign::Minus) => Ordering::Greater,
            (Sign::Minus, Sign::NoSign) => Ordering::Less,
            (Sign::Minus, Sign::Plus) => Ordering::Less,
            (Sign::Minus, Sign::Minus) => self.1.cmp(&other.1).reverse(),
            (Sign::Plus, Sign::Minus) => Ordering::Greater,
            (Sign::Plus, Sign::NoSign) => Ordering::Greater,
            (Sign::Plus, Sign::Plus) => self.1.cmp(&other.1),
        }
    }
}

impl PartialOrd for MI256 {
    fn partial_cmp(&self, other: &MI256) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Div for MI256 {
    type Output = MI256;

    fn div(self, other: MI256) -> MI256 {
        let d = (self.1 / other.1) & SIGN_BIT_MASK.into();

        if d == M256::zero() {
            return MI256::zero();
        }

        match (self.0, other.0) {
            (Sign::Plus, Sign::Plus) |
            (Sign::Minus, Sign::Minus) => MI256(Sign::Plus, d),
            (Sign::Plus, Sign::Minus) |
            (Sign::Minus, Sign::Plus) => MI256(Sign::Minus, d),
            _ => MI256::zero()
        }
    }
}

impl Rem for MI256 {
    type Output = MI256;

    fn rem(self, other: MI256) -> MI256 {
        let r = (self.1 % other.1) & SIGN_BIT_MASK.into();

        if r == M256::zero() {
            return MI256::zero()
        }

        MI256(self.0, r)
    }
}
