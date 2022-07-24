use crate::algebra;
use image::Rgb;
use std::ops::{AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn mix(&self, other: &Color, proportion: f32) -> Color {
        assert!((0.0..=1.0).contains(&proportion));
        Color {
            red: self.red * (1.0 - proportion) + other.red * proportion,
            green: self.green * (1.0 - proportion) + other.green * proportion,
            blue: self.blue * (1.0 - proportion) + other.blue * proportion,
        }
    }

    pub fn desample(&self, samples: u32) -> Color {
        let red = self.red / samples as f32;
        let green = self.green / samples as f32;
        let blue = self.blue / samples as f32;
        Color::new(red, green, blue)
    }

    pub fn gamma_correct(&self) -> Color {
        Color::new(self.red.sqrt(), self.green.sqrt(), self.blue.sqrt())
    }

    pub fn as_image_color(&self) -> Rgb<u8> {
        let red = self.red.min(1.0).max(0.0) * 255.0;
        let green = self.green.min(1.0).max(0.0) * 255.0;
        let blue = self.blue.min(1.0).max(0.0) * 255.0;

        Rgb([red as u8, green as u8, blue as u8])
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }
}

impl Mul<algebra::Scalar> for Color {
    type Output = Color;

    fn mul(self, other: algebra::Scalar) -> Color {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
