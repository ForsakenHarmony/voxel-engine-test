extern crate amethyst;
extern crate cgmath;
extern crate obj;
extern crate genmesh;
extern crate gfx;

mod util;
mod game_state;
mod components;
//mod obj_loader;

use amethyst::engine::{Application};
use amethyst::gfx_device::DisplayConfig;
use amethyst::config::Element;

use components::Object;
use game_state::Example;
//use obj_loader::Obj;

fn main() {
  let path = format!("{}/resources/config.yml", env!("CARGO_MANIFEST_DIR"));
  let display_config = DisplayConfig::from_file(path).unwrap();
  let mut game = Application::build(Example::new(), display_config)
    .register::<Object>()
    .done();
  game.run();
}
