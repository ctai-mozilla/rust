// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct closure_box {
    cl: &fn(),
}

fn box_it(+x: &r/fn()) -> closure_box/&r {
    closure_box {cl: move x}
}

fn call_static_closure(cl: closure_box/&static) {
    (cl.cl)();
}

pub fn main() {
    let cl_box = box_it(|| debug!("Hello, world!"));
    call_static_closure(move cl_box);
}
