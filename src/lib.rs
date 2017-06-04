#[macro_use]
extern crate pixset_derive;

mod font;
mod pix;
mod tileset;

pub use font::Str;
pub use pix::{Pix, PixLike};
pub use tileset::TILESET;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
