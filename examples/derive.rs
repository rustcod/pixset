#[macro_use]
extern crate pixset_derive;
extern crate pixset;

use pixset::PixLike;

#[derive(Copy, Clone, Debug, PixLike)]
#[size(width = "16", height = "16")]
#[total = "4"]
pub enum Tiles {
    A,
    B,
    C,
    Empty,
}

fn main() {
    println!("{:?}", Tiles::pix_order());
    println!("{:?}", Tiles::A.get());
    println!("{:?}", Tiles::B.get());
    println!("{:?}", Tiles::C.get());
    println!("{:?}", Tiles::Empty.get());
}
