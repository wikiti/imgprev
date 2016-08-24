lazy_static! {
  // See this chart: http://www.calmar.ws/vim/256-xterm-24bit-rgb-color-chart.html
  static ref COLORS: Vec<i32> = vec![
    0x080808, 0x121212, 0x1c1c1c, 0x262626, 0x303030, 0x3a3a3a,
    0x444444, 0x4e4e4e, 0x585858, 0x606060, 0x666666, 0x767676,
    0x808080, 0x8a8a8a, 0x949494, 0x9e9e9e, 0xa8a8a8, 0xb2b2b2,
    0xbcbcbc, 0xc6c6c6, 0xd0d0d0, 0xdadada, 0xe4e4e4, 0xeeeeee
  ];
}

use pixel;

fn find_nearest_color(p: &pixel::Pixel) -> usize {
  let value = p.grayscale();
  let mut min: i32 = i32::max_value();
  COLORS.iter().enumerate().map(|(i, c)| (i, (value - c).abs()) )
        .fold(0, |acc, (i, c)| if c < min { min = c; i as i32 } else { acc });

  let index = (COLORS.len() as f64) * (value as f64) / 256.0;
  232 + index as usize
}

pub fn print_pixel(p: &pixel::Pixel) {
  print!("\x1b[48;5;{}m  \x1b[0m", find_nearest_color(p));
}
