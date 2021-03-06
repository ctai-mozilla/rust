// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations and constants for `u16`

use num::NumCast;

mod inst {
    pub type T = u16;
    #[allow(non_camel_case_types)]
    pub type T_SIGNED = i16;
    pub const bits: uint = 16;
}

pub impl u16: NumCast {
    /**
     * Cast `n` to a `u16`
     */
    #[inline(always)]
    static pure fn from<N:NumCast>(n: N) -> u16 { n.to_u16() }

    #[inline(always)] pure fn to_u8(&self)    -> u8    { *self as u8    }
    #[inline(always)] pure fn to_u16(&self)   -> u16   { *self          }
    #[inline(always)] pure fn to_u32(&self)   -> u32   { *self as u32   }
    #[inline(always)] pure fn to_u64(&self)   -> u64   { *self as u64   }
    #[inline(always)] pure fn to_uint(&self)  -> uint  { *self as uint  }

    #[inline(always)] pure fn to_i8(&self)    -> i8    { *self as i8    }
    #[inline(always)] pure fn to_i16(&self)   -> i16   { *self as i16   }
    #[inline(always)] pure fn to_i32(&self)   -> i32   { *self as i32   }
    #[inline(always)] pure fn to_i64(&self)   -> i64   { *self as i64   }
    #[inline(always)] pure fn to_int(&self)   -> int   { *self as int   }

    #[inline(always)] pure fn to_f32(&self)   -> f32   { *self as f32   }
    #[inline(always)] pure fn to_f64(&self)   -> f64   { *self as f64   }
    #[inline(always)] pure fn to_float(&self) -> float { *self as float }
}

#[test]
fn test_numcast() {
    assert (20u   == 20u16.to_uint());
    assert (20u8  == 20u16.to_u8());
    assert (20u16 == 20u16.to_u16());
    assert (20u32 == 20u16.to_u32());
    assert (20u64 == 20u16.to_u64());
    assert (20i   == 20u16.to_int());
    assert (20i8  == 20u16.to_i8());
    assert (20i16 == 20u16.to_i16());
    assert (20i32 == 20u16.to_i32());
    assert (20i64 == 20u16.to_i64());
    assert (20f   == 20u16.to_float());
    assert (20f32 == 20u16.to_f32());
    assert (20f64 == 20u16.to_f64());

    assert (20u16 == NumCast::from(20u));
    assert (20u16 == NumCast::from(20u8));
    assert (20u16 == NumCast::from(20u16));
    assert (20u16 == NumCast::from(20u32));
    assert (20u16 == NumCast::from(20u64));
    assert (20u16 == NumCast::from(20i));
    assert (20u16 == NumCast::from(20i8));
    assert (20u16 == NumCast::from(20i16));
    assert (20u16 == NumCast::from(20i32));
    assert (20u16 == NumCast::from(20i64));
    assert (20u16 == NumCast::from(20f));
    assert (20u16 == NumCast::from(20f32));
    assert (20u16 == NumCast::from(20f64));

    assert (20u16 == num::cast(20u));
    assert (20u16 == num::cast(20u8));
    assert (20u16 == num::cast(20u16));
    assert (20u16 == num::cast(20u32));
    assert (20u16 == num::cast(20u64));
    assert (20u16 == num::cast(20i));
    assert (20u16 == num::cast(20i8));
    assert (20u16 == num::cast(20i16));
    assert (20u16 == num::cast(20i32));
    assert (20u16 == num::cast(20i64));
    assert (20u16 == num::cast(20f));
    assert (20u16 == num::cast(20f32));
    assert (20u16 == num::cast(20f64));
}
