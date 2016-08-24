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

fn find_character(p: &pixel::Pixel) -> &str {
  let value = p.grayscale();
  let index = (COLORS.len() as f64) * (value as f64) / 256.0;
  COLORS[index as usize]
}

pub fn print_pixel(p: &pixel::Pixel) {
  print!("{0}{0}", find_character(p));
}
