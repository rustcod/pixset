#[macro_use]
extern crate pixset_derive;

mod pix;

pub use pix::{Pix, PixLike, PixStr, TILESET};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
