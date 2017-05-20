#[macro_use]
extern crate lazy_static;

mod font;
mod pixset;
mod tileset;

pub use pixset::{Pix, Pixset};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
