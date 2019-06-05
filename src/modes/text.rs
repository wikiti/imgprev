lazy_static! {
  // See this chart: http://www.calmar.ws/vim/256-xterm-24bit-rgb-color-chart.html
  static ref COLORS: Vec<&'static str> = vec![
    " ", ".", "'", "`", "^", "\"", ",", ":", ";", "I",
    "l", "!", "i", ">", "<", "~", "+", "_", "-", "?",
    "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/",
    "t", "f", "j", "r", "x", "n", "u", "v", "c", "z",
    "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z",
    "m", "w", "q", "p", "d", "b", "k", "h", "a", "o",
    "*", "#", "M", "W", "&", "8", "%", "B", "$", "@"
  ];
}

use pixel;
use modes::base::Base;

pub struct Text<'a> {
  pub p: &'a pixel::Pixel
}

impl<'a> Text<'a> {
  fn find_character(&self) -> &str {
    let value = self.p.grayscale();
    let index = (COLORS.len() as f64) * (value as f64) / 256.0;
    COLORS[index as usize]
  }
}

impl<'a> Base for Text<'a> {
  fn print(&self) {
    print!("{0}{0}", self.find_character());
  }
}
