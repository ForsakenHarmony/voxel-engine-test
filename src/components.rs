use amethyst::ecs::{VecStorage, Component};

pub struct Object {
  pub position: [f32; 3],
  pub size: f32
}

impl Object {
  pub fn new() -> Object {
    Object {
      position: [0.0, 0.0, 0.0],
      size: 1.0,
    }
  }
}

impl Component for Object {
  type Storage = VecStorage<Object>;
}
