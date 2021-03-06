// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustc_attrs)]
#![allow(dead_code)]

mod foo {
    pub use self::bar::T;
    mod bar {
        pub trait T {
            fn f(&self) {}
        }
        impl T for () {}
    }
}

fn g() {
    use foo::T;
    ().f(); // Check that this does not trigger a privacy error
}

#[rustc_error]
fn main() {} //~ ERROR compilation successful
