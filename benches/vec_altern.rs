#![feature(test)]
extern crate test;
use test::{Bencher, bench::black_box};

use combin_iterator::{altern, altern::VecAltern};

#[bench]
fn create_100000x2(b: &mut Bencher) {
    b.iter(||{
        black_box(altern!(0..100000, 0..100000));
    });
}


#[bench]
fn create_100000x4(b: &mut Bencher) {
    b.iter(||{
        black_box(altern!(0..100000, 0..100000, 0..100000, 0..100000));
    });
}

#[bench]
fn create_and_count_100000x2(b: &mut Bencher) {
    b.iter(||{
        let iter = altern!(0..100000, 0..100000);
        black_box(iter.count());
    });
}

#[bench]
fn create_and_count_100000x4(b: &mut Bencher) {
    b.iter(||{
        let iter = altern!(0..100000, 0..100000, 0..100000, 0..100000);
        black_box(iter.count());
    });
}


#[bench]
fn create_and_count_100000x8(b: &mut Bencher) {
    b.iter(||{
        let iter =  altern!(0..100000, 0..100000, 0..100000, 0..100000, 0..100000, 0..100000, 0..100000, 0..100000);
        black_box(iter.count());
    });
}

#[bench]
fn create_and_count_50000x16(b: &mut Bencher) {
    b.iter(||{
        let iter =  altern!(0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000, 0..50000);
        black_box(iter.count());
    });
}

#[bench]
fn create_and_count_20000x32(b: &mut Bencher) {
    b.iter(||{
        let mut iter =  VecAltern::new();
        for _ in 0..32 {
            iter = iter.add_and(0..20000);
        }
        black_box(iter.count());
    });
}