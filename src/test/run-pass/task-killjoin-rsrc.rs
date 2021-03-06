// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-win32

// A port of task-killjoin to use a class with a dtor to manage
// the join.

use core::pipes::*;

struct notify {
    ch: Chan<bool>, v: @mut bool,
}

impl Drop for notify {
    fn finalize(&self) {
        error!("notify: task=%? v=%x unwinding=%b b=%b",
               task::get_task(),
               ptr::addr_of(&(*(self.v))) as uint,
               task::failing(),
               *(self.v));
        let b = *(self.v);
        self.ch.send(b);
    }
}

fn notify(ch: Chan<bool>, v: @mut bool) -> notify {
    notify {
        ch: ch,
        v: v
    }
}

fn joinable(f: fn~()) -> Port<bool> {
    fn wrapper(c: Chan<bool>, f: fn()) {
        let b = @mut false;
        error!("wrapper: task=%? allocated v=%x",
               task::get_task(),
               ptr::addr_of(&(*b)) as uint);
        let _r = notify(c, b);
        f();
        *b = true;
    }
    let (p, c) = stream();
    let c = ~mut Some(c);
    do task::spawn_unlinked {
        let mut cc = None;
        *c <-> cc;
        let ccc = option::unwrap(cc);
        wrapper(ccc, f)
    }
    p
}

fn join(port: Port<bool>) -> bool {
    port.recv()
}

fn supervised() {
    // Yield to make sure the supervisor joins before we
    // fail. This is currently not needed because the supervisor
    // runs first, but I can imagine that changing.
    error!("supervised task=%?", task::get_task);
    task::yield();
    fail!();
}

fn supervisor() {
    error!("supervisor task=%?", task::get_task());
    let t = joinable(supervised);
    join(t);
}

pub fn main() {
    join(joinable(supervisor));
}

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
