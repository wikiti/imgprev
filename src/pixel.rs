pub struct Pixel {
  pub r: i32,
  pub g: i32,
  pub b: i32
}

impl Pixel {
  pub fn distance(&self, p: Pixel) -> i32 {
    let sum = (self.r - p.r).pow(2) + (self.g - p.g).pow(2) + (self.b - p.b).pow(2);
    (sum as f64).sqrt() as i32
  }

  pub fn grayscale(&self) -> i32 {
    (0.29 * (self.r as f64) + 0.6 * (self.g as f64) + 0.11 * (self.b as f64)).floor() as i32
  }
}

pub fn build(value: i32) -> Pixel {
  Pixel { r: (value >> 16) & 0xFF, g: (value >> 8) & 0xFF, b: value & 0xFF }
}
