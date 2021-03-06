use amethyst::renderer::{VertexPosNormal};
use amethyst::components::rendering::Renderable;
use amethyst::ecs::{World};

use genmesh::generators::Cube;
use genmesh::generators::Plane;
use genmesh::{MapToVertices, Triangulate, Vertices, Quad};

use cgmath::{Vector3, InnerSpace};

use components::Object;

fn gen_vertex(pos: [f32; 3], tex: [f32; 2]) -> VertexPosNormal {
  let x = pos[0];
  let y = pos[1];
  let z = pos[2];
  VertexPosNormal {
    pos: [x / 2., y / 2., z / 2.],
    normal: Vector3::new(x, y, z).normalize().into(),
    tex_coord: tex,
  }
}

pub fn gen_rectangle(w: f32, h: f32) -> Vec<VertexPosNormal> {
  let data: Vec<VertexPosNormal> = vec![
    VertexPosNormal {
      pos: [-w / 2., -h / 2., 0.],
      normal: [0., 1., 0.],
      tex_coord: [0., 0.],
    },
    VertexPosNormal {
      pos: [w / 2., -h / 2., 0.],
      normal: [0., 1., 0.],
      tex_coord: [1., 0.],
    },
    VertexPosNormal {
      pos: [w / 2., h / 2., 0.],
      normal: [0., 1., 0.],
      tex_coord: [1., 1.],
    },
    VertexPosNormal {
      pos: [w / 2., h / 2., 0.],
      normal: [0., 1., 0.],
      tex_coord: [1., 1.],
    },
    VertexPosNormal {
      pos: [-w / 2., h / 2., 0.],
      normal: [0., 1., 0.],
      tex_coord: [1., 1.],
    },
    VertexPosNormal {
      pos: [-w / 2., -h / 2., 0.],
      normal: [0., 1., 0.],
      tex_coord: [1., 1.],
    },
  ];
  data
}

pub fn gen_plane() -> Vec<VertexPosNormal> {
  let vertex_data: Vec<VertexPosNormal> = Plane::new()
    .vertex(|(x, z)| gen_vertex([x, 0., z], [0., 0.]))
    .map(|Quad { x: v0, y: v1, z: v2, w: v3 }| {
      Quad::new(gen_vertex(v0.pos, [0., 0.]),
                gen_vertex(v1.pos, [1., 0.]),
                gen_vertex(v2.pos, [1., 1.]),
                gen_vertex(v3.pos, [0., 1.]))
    })
    .triangulate()
    .vertices()
    .collect();
  
  vertex_data
}

pub fn gen_cube() -> Vec<VertexPosNormal> {
  let vertex_data: Vec<VertexPosNormal> = Cube::new()
    .vertex(|(x, y, z)| gen_vertex([x, y, z], [0., 0.]))
    .map(|Quad { x: v0, y: v1, z: v2, w: v3 }| {
      Quad::new(gen_vertex(v0.pos, [0., 0.]),
                gen_vertex(v1.pos, [1., 0.]),
                gen_vertex(v2.pos, [1., 1.]),
                gen_vertex(v3.pos, [0., 1.]))
    })
    .triangulate()
    .vertices()
    .collect();
  
  vertex_data
}

pub fn create_cube(x: f32, y: f32, z: f32, world: &mut World, cube: Renderable) {
  use amethyst::components::transform::{Transform};
  
  let mut cube_obj = Object::new();
  cube_obj.position = [x, y, z];
  world.create_now()
       .with(cube)
       .with(cube_obj.get_transform())
       .with(cube_obj)
       .with(Transform::default())
       .build();
}
