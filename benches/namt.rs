#![feature(test)]

extern crate test;

use namt::{find_unsafe_number, Number};
use test::Bencher;

#[bench]
fn bench_unsafe_number(b: &mut Bencher) {
    let numbers: Vec<Number> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse().expect("should parse number"))
        .collect();

    b.iter(|| find_unsafe_number(100, &numbers));
}
