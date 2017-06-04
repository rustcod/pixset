#[macro_use]
extern crate pixset_derive;

mod font;
mod pix;
mod tileset;
mod traits;

pub use font::Str;
pub use pix::Pix;
pub use tileset::TILESET;
pub use traits::PixLike;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
