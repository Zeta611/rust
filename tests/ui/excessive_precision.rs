// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(tool_lints)]
#![warn(clippy::excessive_precision)]
#![allow(clippy::print_literal)]

fn main() {
    // Consts
    const GOOD32: f32 = 0.123_456;
    const GOOD32_SM: f32 = 0.000_000_000_1;
    const GOOD32_DOT: f32 = 10_000_000_000.0;
    const GOOD32_EDGE: f32 = 1.000_000_8;
    const GOOD64: f64 = 0.123_456_789_012;
    const GOOD64_SM: f32 = 0.000_000_000_000_000_1;
    const GOOD64_DOT: f32 = 10_000_000_000_000_000.0;

    const BAD32_1: f32 = 0.123_456_789_f32;
    const BAD32_2: f32 = 0.123_456_789;
    const BAD32_3: f32 = 0.100_000_000_000_1;
    const BAD32_EDGE: f32 = 1.000_000_9;

    const BAD64_1: f64 = 0.123_456_789_012_345_67f64;
    const BAD64_2: f64 = 0.123_456_789_012_345_67;
    const BAD64_3: f64 = 0.100_000_000_000_000_000_1;

    // Literal as param
    println!("{:?}", 8.888_888_888_888_888_888_888);

    // // TODO add inferred type tests for f32
    // Locals
    let good32: f32 = 0.123_456_f32;
    let good32_2: f32 = 0.123_456;

    let good64: f64 = 0.123_456_789_012;
    let good64_suf: f64 = 0.123_456_789_012f64;
    let good64_inf = 0.123_456_789_012;

    let bad32: f32 = 1.123_456_789;
    let bad32_suf: f32 = 1.123_456_789_f32;
    let bad32_inf = 1.123_456_789_f32;

    let bad64: f64 = 0.123_456_789_012_345_67;
    let bad64_suf: f64 = 0.123_456_789_012_345_67f64;
    let bad64_inf = 0.123_456_789_012_345_67;

    // Vectors
    let good_vec32: Vec<f32> = vec![0.123_456];
    let good_vec64: Vec<f64> = vec![0.123_456_789];

    let bad_vec32: Vec<f32> = vec![0.123_456_789];
    let bad_vec64: Vec<f64> = vec![0.123_456_789_123_456_789];

    // Exponential float notation
    let good_e32: f32 = 1e-10;
    let bad_e32: f32 = 1.123_456_788_888e-10;

    let good_bige32: f32 = 1E-10;
    let bad_bige32: f32 = 1.123_456_788_888E-10;

    // Inferred type
    let good_inferred: f32 = 1f32 * 1_000_000_000.;

    // issue #2840
    let num = 0.000_000_000_01e-10f64;
}
