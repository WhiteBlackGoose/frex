pub trait Colorizer {
    fn from_iter(&self, iter: u32, max_iter: u32) -> raylib::core::color::Color;
}

pub struct ColorizerHue {}

impl Colorizer for ColorizerHue {
    fn from_iter(&self, iter: u32, max_iter: u32) -> raylib::core::color::Color {
        // https://www.rapidtables.com/convert/color/hsv-to-rgb.html
        // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB
        let h = iter as f32 / max_iter as f32;
        let hi = (h * 6.0).floor() as u32;
        let s = 1.0;
        let v = 0.7;
        let f = h * 6.0 - hi as f32;

        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        let (rr, gg, bb) = match hi {
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            5 => (v, p, q),
            _ => (v, t, p),
        };
        raylib::color::Color::color_from_hsv(h * 255.0, s, v)
    }
}
