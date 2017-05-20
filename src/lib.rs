#[macro_use]
extern crate lazy_static;

mod pixset;

pub use pixset::{PIXSET, Pix, Pixset};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
