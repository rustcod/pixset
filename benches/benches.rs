#![feature(test)]

extern crate pixset;
extern crate test;

use test::Bencher;

#[bench]
fn bench_font_iter(b: &mut Bencher) {
    b.iter(|| {
        for (_pix, _offset) in pixset::Str::from("Yo, Dawg;").iter() {}
    })
}

#[bench]
fn bench_pixset_get(b: &mut Bencher) {
    let pixset = pixset::Pixset::new(100, 16);
    b.iter(|| pixset.get(&pixset::Pix::A))
}
