#![feature(test)]

extern crate pixset;
extern crate test;

use test::Bencher;

#[bench]
fn bench_font_iter(b: &mut Bencher) {
    b.iter(|| for (_pix, _offset) in pixset::Str::from("Yo, Dawg;").iter() {},)
}
