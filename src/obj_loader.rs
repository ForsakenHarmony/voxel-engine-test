use obj;

use amethyst::asset_manager::{Assets, AssetLoader, AssetLoaderRaw};
use amethyst::components::rendering::{Mesh};
use amethyst::renderer::{VertexPosNormal};

use cgmath::{InnerSpace, Vector3};

use std::io::BufReader;

pub struct Obj(obj::Obj);

impl AssetLoaderRaw for Obj {
  fn from_raw(_: &Assets, data: &[u8]) -> Option<Obj> {
    obj::load_obj(BufReader::new(data)).ok().map(|obj| Obj(obj))
  }
}

impl AssetLoader<Mesh> for Obj {
  fn from_data(assets: &mut Assets, obj: Obj) -> Option<Mesh> {
    let obj = obj.0;
    let vertices = obj.indices.iter().map(|&index| {
      let vertex = obj.vertices[index as usize];
      let normal = vertex.normal;
      let normal = Vector3::from(normal).normalize();
      VertexPosNormal {
        pos: vertex.position,
        normal: normal.into(),
        tex_coord: [0., 0.],
      }
    }).collect::<Vec<VertexPosNormal>>();
    
    AssetLoader::<Mesh>::from_data(assets, vertices)
  }
}
