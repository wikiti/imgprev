#[macro_use] extern crate lazy_static;
#[macro_use] extern crate image;
#[macro_use] extern crate clap;

mod modes;
mod pixel;
mod loader;

use clap::App;

fn main() {
  let yaml = load_yaml!("../cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let printer = match matches.value_of("mode").unwrap() {
    "ansi"       => modes::ansi::print_pixel,
    "grayscale"  => modes::grayscale::print_pixel,
    "true_color" => modes::true_color::print_pixel,
    _            => modes::text::print_pixel
  };

  let mut l = loader::ImageLoader { filename: matches.value_of("FILE").unwrap(),
                                    cols:     matches.value_of("width").unwrap().parse::<u32>().unwrap(),
                                    filter:   matches.value_of("filter").unwrap(),
                                    ..Default::default() };
  l.load();
  l.paint(printer);
}
