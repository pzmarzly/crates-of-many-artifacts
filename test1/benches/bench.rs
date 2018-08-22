#![feature(test)]
extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_pow(b: &mut Bencher) {
    b.iter(|| {
        for i in 1..100 {
            black_box(2 * 2);
        }
    });
}
