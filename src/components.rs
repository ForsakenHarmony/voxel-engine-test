use cgmath::{Quaternion, Euler, Deg, Vector3, Rotation};
use amethyst::ecs::{VecStorage, Component};
use amethyst::components::transform::LocalTransform;
use std::ops::DerefMut;

pub struct Object {
  pub position: [f32; 3],
  //TODO: maybe switch to a vector?
  pub rotation: [f32; 3],
  pub size: f32
}

impl Object {
  pub fn new() -> Object {
    Object {
      position: [0., 0., 0.],
      rotation: [0., 0., 0.],
      size: 1.,
    }
  }
  
  pub fn get_transform(&self) -> LocalTransform {
    let mut transform = LocalTransform::default();
    {
      let inner = transform.deref_mut();
      
      let a: Vec<Deg<f32>> = self.rotation.iter().map(|&x| Deg(x)).collect();
      
      inner.translation = self.position;
      inner.rotation = Quaternion::from(Euler::new(a[0],a[1],a[2])).into();
//      inner.rotation = Quaternion::look_at(Vector3::new(0., 1., 0.), Vector3::new(0., 1., 0.)).into();
//      inner.rotation = [1., 0., 0., 0.];
      inner.scale = [self.size, self.size, self.size];
    }
    transform
  }
}

impl Component for Object {
  type Storage = VecStorage<Object>;
}
