// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_unsafe)]
#![allow(dead_code)]
#![deny(unsafe_code)]

use std::marker::PhantomFn;

struct Bar;

#[allow(unsafe_code)]
mod allowed_unsafe {
    use std::marker::PhantomFn;
    fn allowed() { unsafe {} }
    unsafe fn also_allowed() {}
    unsafe trait AllowedUnsafe : PhantomFn<Self> {}
    unsafe impl AllowedUnsafe for super::Bar {}
}

macro_rules! unsafe_in_macro {
    () => {
        unsafe {} //~ ERROR: usage of an `unsafe` block
    }
}

unsafe fn baz() {} //~ ERROR: declaration of an `unsafe` function
unsafe trait Foo : PhantomFn<Self> {} //~ ERROR: declaration of an `unsafe` trait
unsafe impl Foo for Bar {} //~ ERROR: implementation of an `unsafe` trait

trait Baz {
    unsafe fn baz(&self); //~ ERROR: declaration of an `unsafe` method
    unsafe fn provided(&self) {} //~ ERROR: implementation of an `unsafe` method
    unsafe fn provided_override(&self) {} //~ ERROR: implementation of an `unsafe` method
}

impl Baz for Bar {
    unsafe fn baz(&self) {} //~ ERROR: implementation of an `unsafe` method
    unsafe fn provided_override(&self) {} //~ ERROR: implementation of an `unsafe` method
}

fn main() {
    unsafe {} //~ ERROR: usage of an `unsafe` block

    unsafe_in_macro!()
}
