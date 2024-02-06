#![feature(test)]
extern crate test;
use test::{Bencher, bench::black_box};

use combin_iterator::iterators::BiAltern;

#[bench]
fn create_100000x2(b: &mut Bencher) {
    b.iter(||{
        black_box(BiAltern::new(0..100000, 0..100000));
    });
}

#[bench]
fn create_100000x4(b: &mut Bencher) {
    b.iter(||{
        black_box(BiAltern::new(BiAltern::new(0..100000, 0..100000), BiAltern::new(0..100000, 0..100000)));
    });
}


#[bench]
fn create_and_count_100000x2(b: &mut Bencher) {
    b.iter(||{
        let iter = BiAltern::new(0..100000, 0..100000);
        black_box(iter.count());
    });
}

#[bench]
fn create_and_count_100000x4(b: &mut Bencher) {
    b.iter(||{
        let iter = BiAltern::new(BiAltern::new(0..100000, 0..100000), BiAltern::new(0..100000, 0..100000));
        black_box(iter.count());
    });
}

#[bench]
fn create_and_count_100000x8(b: &mut Bencher) {
    b.iter(||{
        let iter = BiAltern::new(BiAltern::new(BiAltern::new(0..100000, 0..100000), BiAltern::new(0..100000, 0..100000)), BiAltern::new(BiAltern::new(0..100000, 0..100000), BiAltern::new(0..100000, 0..100000)));
        black_box(iter.count());
    });
}

#[bench]
fn create_and_count_50000x16(b: &mut Bencher) {
    b.iter(||{
        let iter1 = BiAltern::new(BiAltern::new(BiAltern::new(0..50000, 0..50000), BiAltern::new(0..50000, 0..50000)), BiAltern::new(BiAltern::new(0..50000, 0..50000), BiAltern::new(0..50000, 0..50000)));
        let iter2 = BiAltern::new(BiAltern::new(BiAltern::new(0..50000, 0..50000), BiAltern::new(0..50000, 0..50000)), BiAltern::new(BiAltern::new(0..50000, 0..50000), BiAltern::new(0..50000, 0..50000)));
        let iter = BiAltern::new(iter1, iter2);
        black_box(iter.count());
    });
}

#[bench]
fn create_and_count_20000x32(b: &mut Bencher) {
    b.iter(||{
        let iter1 = BiAltern::new(BiAltern::new(BiAltern::new(0..20000, 0..20000), BiAltern::new(0..20000, 0..20000)), BiAltern::new(BiAltern::new(0..20000, 0..20000), BiAltern::new(0..20000, 0..20000)));
        let iter2 = BiAltern::new(BiAltern::new(BiAltern::new(0..20000, 0..20000), BiAltern::new(0..20000, 0..20000)), BiAltern::new(BiAltern::new(0..20000, 0..20000), BiAltern::new(0..20000, 0..20000)));
        let iter3 = BiAltern::new(iter1, iter2);

        let iter4 = BiAltern::new(BiAltern::new(BiAltern::new(0..20000, 0..20000), BiAltern::new(0..20000, 0..20000)), BiAltern::new(BiAltern::new(0..20000, 0..20000), BiAltern::new(0..20000, 0..20000)));
        let iter5 = BiAltern::new(BiAltern::new(BiAltern::new(0..20000, 0..20000), BiAltern::new(0..20000, 0..20000)), BiAltern::new(BiAltern::new(0..20000, 0..20000), BiAltern::new(0..20000, 0..20000)));
        let iter6 = BiAltern::new(iter4, iter5);

        let iter = BiAltern::new(iter3, iter6);
        black_box(iter.count());
    });
}