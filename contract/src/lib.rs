#![no_std]
#![no_main]

use common::params::{CtorParams, DtorParams};

#[no_mangle]
#[allow(non_snake_case)]
fn Ctor(_params: *const CtorParams) {}

#[no_mangle]
#[allow(non_snake_case)]
fn Dtor(_params: *const DtorParams) {}

#[no_mangle]
#[allow(non_snake_case)]
fn Method_2(_params: *const usize) {}
