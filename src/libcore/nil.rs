// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!

Functions for the unit type.

*/

use cmp::{Eq, Ord};

#[cfg(notest)]
impl Eq for () {
    #[inline(always)]
    pure fn eq(&self, _other: &()) -> bool { true }
    #[inline(always)]
    pure fn ne(&self, _other: &()) -> bool { false }
}

#[cfg(notest)]
impl Ord for () {
    #[inline(always)]
    pure fn lt(&self, _other: &()) -> bool { false }
    #[inline(always)]
    pure fn le(&self, _other: &()) -> bool { true }
    #[inline(always)]
    pure fn ge(&self, _other: &()) -> bool { true }
    #[inline(always)]
    pure fn gt(&self, _other: &()) -> bool { false }
}

