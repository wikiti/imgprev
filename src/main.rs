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

  let mut l = loader::ImageLoader { filename: matches.value_of("FILE").unwrap(),
                                    mode:     matches.value_of("mode").unwrap(),
                                    cols:     matches.value_of("width").unwrap().parse::<u32>().unwrap(),
                                    filter:   matches.value_of("filter").unwrap(),
                                    ..Default::default() };
  l.load();
  l.paint();
}
