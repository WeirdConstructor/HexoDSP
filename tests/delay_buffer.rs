// Copyright (c) 2021 Weird Constructor <weirdconstructor@gmail.com>
// This file is a part of HexoDSP. Released under GPL-3.0-or-later.
// See README.md and COPYING for details.

mod common;
use common::*;

#[test]
fn check_delaybuffer_linear_interpolation() {
    let mut buf = crate::helpers::DelayBuffer::new();

    buf.feed(0.0);
    buf.feed(0.1);
    buf.feed(0.2);
    buf.feed(0.3);
    buf.feed(0.4);
    buf.feed(0.5);
    buf.feed(0.6);
    buf.feed(0.7);
    buf.feed(0.8);
    buf.feed(0.9);
    buf.feed(1.0);

    let mut samples_out = vec![];
    let mut pos = 0.0;
    let pos_inc = 0.5;
    for _ in 0..20 {
        samples_out.push(buf.linear_interpolate_at_s(pos));
        pos += pos_inc;
    }

    assert_vec_feq!(
        samples_out,
        vec![
            1.0, 0.95, 0.9, 0.85, 0.8, 0.75, 0.7, 0.65, 0.6, 0.55, 0.5, 0.45, 0.4, 0.35000002, 0.3,
            0.25, 0.2, 0.15, 0.1, 0.05
        ]
    );

    let mut samples_out = vec![];
    let mut pos = 0.0;
    let pos_inc = 0.2;
    for _ in 0..30 {
        samples_out.push(buf.linear_interpolate_at_s(pos));
        pos += pos_inc;
    }

    assert_vec_feq!(
        samples_out,
        vec![
            1.0, 0.98, 0.96, 0.94, 0.91999996, 0.9, 0.88, 0.85999995, 0.84, 0.82, 0.8, 0.78, 0.76,
            0.73999995, 0.71999997, 0.6999999, 0.67999995, 0.65999997, 0.6399999, 0.61999995,
            0.59999996, 0.58, 0.56, 0.54, 0.52000004, 0.50000006, 0.48000008, 0.4600001,
            0.44000012, 0.42000014
        ]
    );
}

#[test]
fn check_delaybuffer_nearest() {
    let mut buf = crate::helpers::DelayBuffer::new();

    buf.feed(0.0);
    buf.feed(0.1);
    buf.feed(0.2);
    buf.feed(0.3);
    buf.feed(0.4);
    buf.feed(0.5);
    buf.feed(0.6);
    buf.feed(0.7);
    buf.feed(0.8);
    buf.feed(0.9);
    buf.feed(1.0);

    let mut samples_out = vec![];
    let mut pos = 0.0;
    let pos_inc = 0.5;
    for _ in 0..20 {
        samples_out.push(buf.at(pos as usize));
        pos += pos_inc;
    }

    assert_vec_feq!(
        samples_out,
        vec![
            1.0, 1.0, 0.9, 0.9, 0.8, 0.8, 0.7, 0.7, 0.6, 0.6, 0.5, 0.5, 0.4, 0.4, 0.3, 0.3, 0.2,
            0.2, 0.1, 0.1
        ]
    );

    let mut samples_out = vec![];
    let mut pos = 0.0;
    let pos_inc = 0.2;
    for _ in 0..30 {
        samples_out.push(buf.at(pos as usize));
        pos += pos_inc;
    }

    assert_vec_feq!(
        samples_out,
        vec![
            1.0, 1.0, 1.0, 1.0, 1.0, 0.9, 0.9, 0.9, 0.9, 0.9, 0.9, 0.8, 0.8, 0.8, 0.8, 0.7, 0.7,
            0.7, 0.7, 0.7, 0.6, 0.6, 0.6, 0.6, 0.6, 0.5, 0.5, 0.5, 0.5, 0.5
        ]
    );
}

#[test]
fn check_cubic_interpolate() {
    use crate::helpers::cubic_interpolate;
    let data = [1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];

    let mut samples_out = vec![];
    let mut pos = 0.0_f32;
    let pos_inc = 0.5_f32;
    for _ in 0..30 {
        let i = pos.floor() as usize;
        let f = pos.fract();
        samples_out.push(cubic_interpolate(&data[..], data.len(), i, f));
        pos += pos_inc;
    }
    assert_vec_feq!(
        samples_out,
        vec![
            1.0,
            1.01875,
            0.9,
            0.85,
            0.8,
            0.75,
            0.7,
            0.65,
            0.6,
            0.55,
            0.5,
            0.45,
            0.4,
            0.35000002,
            0.3,
            0.25,
            0.2,
            0.15,
            0.1,
            -0.018750004,
            0.0,
            0.49999997,
            1.0,
            1.01875,
            0.9,
            0.85,
            0.8,
            0.75,
            0.7,
            0.65
        ]
    );

    let mut samples_out = vec![];
    let mut pos = 0.0_f32;
    let pos_inc = 0.1_f32;
    for _ in 0..30 {
        let i = pos.floor() as usize;
        let f = pos.fract();
        samples_out.push(cubic_interpolate(&data[..], data.len(), i, f));
        pos += pos_inc;
    }
    assert_vec_feq!(
        samples_out,
        vec![
            1.0, 1.03455, 1.0504, 1.05085, 1.0392, 1.01875, 0.99279994, 0.9646499, 0.9375999,
            0.91494995, 0.9, 0.89, 0.87999994, 0.86999995, 0.85999995, 0.84999996, 0.84, 0.83,
            0.82, 0.80999994, 0.8, 0.79, 0.78000003, 0.77000004, 0.76, 0.75, 0.74, 0.73, 0.72,
            0.71000004
        ]
    );
}

#[test]
fn check_delaybuffer_cubic_interpolation() {
    let mut buf = crate::helpers::DelayBuffer::new();

    buf.feed(0.0);
    buf.feed(0.1);
    buf.feed(0.2);
    buf.feed(0.3);
    buf.feed(0.4);
    buf.feed(0.5);
    buf.feed(0.6);
    buf.feed(0.7);
    buf.feed(0.8);
    buf.feed(0.9);
    buf.feed(1.0);

    let mut samples_out = vec![];
    let mut pos = 0.0;
    let pos_inc = 0.1;
    for _ in 0..30 {
        samples_out.push(buf.cubic_interpolate_at_s(pos));
        pos += pos_inc;
    }

    assert_vec_feq!(
        samples_out,
        vec![
            1.0, 1.03455, 1.0504, 1.05085, 1.0392, 1.01875, 0.99279994, 0.9646499, 0.9375999,
            0.91494995, 0.9, 0.89, 0.87999994, 0.86999995, 0.85999995, 0.84999996, 0.84, 0.83,
            0.82, 0.80999994, 0.8, 0.79, 0.78000003, 0.77000004, 0.76, 0.75, 0.74, 0.73, 0.72,
            0.71000004
        ]
    );

    let mut samples_out = vec![];
    let mut pos = 0.0;
    let pos_inc = 0.5;
    for _ in 0..30 {
        samples_out.push(buf.cubic_interpolate_at_s(pos));
        pos += pos_inc;
    }

    assert_vec_feq!(
        samples_out,
        vec![
            1.0,
            1.01875,
            0.9,
            0.85,
            0.8,
            0.75,
            0.7,
            0.65,
            0.6,
            0.55,
            0.5,
            0.45,
            0.4,
            0.35000002,
            0.3,
            0.25,
            0.2,
            0.15,
            0.1,
            0.043750003,
            0.0,
            -0.00625,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        ]
    );
}
