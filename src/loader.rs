use pixel;
use std::path::Path;
use image;
use image::GenericImage;

pub struct ImageLoader<'a> {
  pub filename: &'a str,
  pub pixels: Vec<pixel::Pixel>,
  pub width: u32,
  pub height: u32,
  pub cols: u32,
  pub filter: &'a str
}

impl<'a> ImageLoader<'a> {
  pub fn load(&mut self) {
    // load image
    let mut img = image::open(&Path::new(self.filename)).unwrap();
    let (real_width, real_height) = img.dimensions();

    // resize it to fit terminal
    let scale = self.cols as f64 / real_width as f64;
    let (width, height) = ( (scale * real_width as f64) as u32, (scale * real_height as f64) as u32 );

    if real_width > self.cols { img = img.resize( width, height, self.filter()); }
    let (width, height) = img.dimensions();

    self.width = width;
    self.height = height;

    // load pixels
    for x in 0..self.width {
      for y in 0..self.height {
        let value = img.get_pixel(x, y);
        self.pixels.push(pixel::Pixel { r: value[0] as i32, g: value[1] as i32, b: value[2] as i32 });
      }
    }
  }

  pub fn paint<F>(&mut self, f: F) where F: Fn(&pixel::Pixel) {
    for y in 0..self.height {
      for x in 0..self.width {
        f(self.get_pixel(x, y));
      }
      self.newline();
    }
    self.newline();
  }

  fn filter(&self) -> image::imageops::FilterType {
    match self.filter {
      "linear"   => image::imageops::FilterType::Triangle,
      "cubic"    => image::imageops::FilterType::CatmullRom,
      "gaussian" => image::imageops::FilterType::Gaussian,
      "lanczos3" => image::imageops::FilterType::Lanczos3,
      _          => image::imageops::FilterType::Nearest
    }
  }

  fn get_pixel(&mut self, x: u32, y: u32) -> &pixel::Pixel {
    &self.pixels[(self.height * x + y) as usize]
  }

  fn newline(&self) {
    print!("\n");
  }
}

impl<'a> Default for ImageLoader<'a> {
  fn default () -> ImageLoader<'a> {
    ImageLoader { filename: "", pixels: vec![], width: 0, height: 0, cols: 50, filter: "none" }
  }
}
