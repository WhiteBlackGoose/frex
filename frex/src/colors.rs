pub trait Colorizer {
    fn from_iter(&self, iter: u32, max_iter: u32) -> raylib::core::color::Color;
}
