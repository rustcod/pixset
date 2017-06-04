pub trait PixLike: Default {
    fn pix_order() -> Vec<Self>;
    fn tile_size(&self) -> i32;
    fn get(&self) -> (f32, f32, f32, f32);
}
