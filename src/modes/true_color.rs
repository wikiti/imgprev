use pixel;
use modes::base::Base;

pub struct TrueColor<'a> {
  pub p: &'a pixel::Pixel
}

impl<'a> Base for TrueColor<'a> {
  fn print(&self) {
    print!("\x1b[48;2;{};{};{}m  \x1b[0m", self.p.r, self.p.g, self.p.b);
  }
}
