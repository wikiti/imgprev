use pixel;

pub fn print_pixel(p: &pixel::Pixel) {
  print!("\x1b[48;2;{};{};{}m  \x1b[0m", p.r, p.g, p.b);
}
