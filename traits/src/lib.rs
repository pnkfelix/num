// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Numeric traits for generic mathematics

use std::ops::{Add, Sub, Mul, Div, Rem};

pub use bounds::Bounded;
pub use float::Float;
pub use identities::{Zero, One};
pub use ops::checked::*;
pub use ops::saturating::Saturating;
pub use sign::{Signed, Unsigned};
pub use int::PrimInt;
pub use from_str::*;
pub use cast::*;

mod identities;
mod sign;
mod ops;
mod bounds;
mod float;
mod int;
mod from_str;
mod cast;

/// The base trait for numeric types
pub trait Num: PartialEq + Zero + One
    + Add<Output = Self> + Sub<Output = Self>
    + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self> + FromStrRadix
{
    /// Convert from a string and radix <= 36.
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, <Self as FromStrRadix>::Error> {
        FromStrRadix::from_str_radix(str, radix)
    }
}

impl<T> Num for T
where T: PartialEq + Zero + One + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Rem<Output=T> + FromStrRadix {}
