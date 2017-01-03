use amethyst::engine::{State, Trans};
use amethyst::ecs::{World};
use amethyst::asset_manager::{AssetManager, DirectoryStore};
use amethyst::event::WindowEvent;
use amethyst::renderer::{Pipeline};
use amethyst::components::rendering::{Mesh, Texture, TextureLoadData};
use amethyst::world_resources::camera::{Camera};

use gfx::tex::{AaMode, Kind};

use components::Object;

use util::*;

pub struct Example {
  t: f32,
  c: u32,
  fps: f32,
}

impl Example {
  pub fn new() -> Example {
    Example {
      t: 0.,
      c: 0,
      fps: 0.,
    }
  }
}

impl State for Example {
  fn on_start(&mut self, world: &mut World, asset_manager: &mut AssetManager, pipeline: &mut Pipeline) {
    use amethyst::renderer::pass::{Clear, DrawShaded};
    use amethyst::renderer::{Layer, Light};
    use amethyst::components::transform::{Transform};
    use amethyst::world_resources::camera::{Projection};
    use amethyst::world_resources::ScreenDimensions;
    
    {
      let dimensions = world.read_resource::<ScreenDimensions>();
      let mut camera = world.write_resource::<Camera>();
      let proj = Projection::Perspective {
        fov: 90.0,
        aspect_ratio: dimensions.aspect_ratio,
        near: 0.5,
        far: 100.0,
      };
      camera.projection = proj;
      camera.eye = [-3.0, 5.0, 0.0];
      camera.target = [0.0, 0.0, 0.0];
      camera.up = [0.0, 1.0, 0.0];
    }
    
    asset_manager.register_asset::<Mesh>();
    asset_manager.register_asset::<Texture>();
    
    let assets_path = format!("{}/resources/assets", env!("CARGO_MANIFEST_DIR"));
    asset_manager.register_store(DirectoryStore::new(assets_path));
    
    // Create some colors for the world.
    asset_manager.load_asset_from_data::<Texture, [f32; 4]>("crate", [0.4, 0.2, 0.0, 1.0]);
    asset_manager.load_asset_from_data::<Texture, [f32; 4]>("tan", [0.7, 0.5, 0.3, 1.0]);
    asset_manager.load_asset_from_data::<Texture, [f32; 4]>("black", [0.0, 0.0, 0.0, 1.0]);
    asset_manager.load_asset_from_data::<Texture, [f32; 4]>("gray", [0.3, 0.3, 0.3, 1.0]);
    asset_manager.load_asset_from_data::<Texture, [f32; 4]>("green", [0.7, 1.0, 0.7, 1.0]);
    
    asset_manager.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
    asset_manager.load_asset_from_data::<Texture, [f32; 4]>("grey", [0.01, 0.01, 0.01, 1.0]);
    
    // Create a manual greenish texture
    let data = TextureLoadData {
      kind: Kind::D2(4, 2, AaMode::Single),
      raw: &[
        &[[0, 92, 9, 255], [0, 104, 10, 255], [0, 123, 12, 255], [1, 142, 14, 255]],
        &[[0, 104, 10, 255], [0, 123, 12, 255], [1, 142, 14, 255], [1, 166, 17, 255]],
      ],
    };
    asset_manager.load_asset_from_data::<Texture, TextureLoadData>("grassy", data);
    
    //    let square_vertices = gen_rectangle(1.0, 1.0);
    //    asset_manager.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", square_vertices);
    //    let square = asset_manager.create_renderable("square", "white", "grey").unwrap();
    //
    //    let mut plane = Object::new();
    //    world.create_now()
    //         .with(square)
    //         .with(plane)
    //         .with(LocalTransform::default())
    //         .with(Transform::default())
    //         .build();
    
    let cube_vertices = gen_cube();
    asset_manager.load_asset_from_data::<Mesh, Vec<_>>("cube", cube_vertices);
    let cube = asset_manager.create_renderable("cube", "grey", "white").unwrap();
    
    create_cube(0., 2., 0., world, cube.clone());
    
//    let mut cube_obj = Object::new();
//    cube_obj.size = 1.;
//    cube_obj.position[1] = 2.0;
//    world.create_now()
//         .with(cube)
//         .with(cube_obj.get_transform())
//         .with(cube_obj)
//         .with(Transform::default())
//         .build();
    
    let plane_vertices = gen_plane();
    asset_manager.load_asset_from_data::<Mesh, Vec<_>>("plane", plane_vertices);
    let plane = asset_manager.create_renderable("plane", "grey", "grassy").unwrap();
    
    let mut plane_obj = Object::new();
    plane_obj.size = 8.;
    world.create_now()
         .with(plane)
         .with(plane_obj.get_transform())
         .with(plane_obj)
         .with(Transform::default())
         .build();
    
    //Sun
    let light = Light {
      center: [0.0, 0.0, 4.0],
      radius: 100.0,
      color: [1.0, 1.0, 0.95, 1.0],
      propagation_constant: 50.,
      propagation_linear: 0.3,
      propagation_r_square: 0.6,
    };
    
    world.create_now()
         .with(light)
         .build();
    
    //RGB Lights
    let light = Light {
      center: [-3.0, 3.0, -3.0],
      radius: 20.0,
      color: [0.0, 0.0, 1.0, 1.0],
      propagation_constant: 0.2,
      propagation_linear: 0.2,
      propagation_r_square: 0.6,
    };
    
    world.create_now()
         .with(light)
         .build();
    
    let light = Light {
      center: [3.0, 3.0, -3.0],
      radius: 20.0,
      color: [0.0, 1.0, 0.0, 1.0],
      propagation_constant: 0.2,
      propagation_linear: 0.2,
      propagation_r_square: 0.6,
    };
    
    world.create_now()
         .with(light)
         .build();
    
    let light = Light {
      center: [0.0, 3.0, 3.0],
      radius: 20.0,
      color: [1.0, 0.0, 0.0, 1.0],
      propagation_constant: 0.2,
      propagation_linear: 0.2,
      propagation_r_square: 0.6,
    };
    
    world.create_now()
         .with(light)
         .build();
    
    let layer =
    Layer::new("main",
               vec![
                 Clear::new([0.15, 0.15, 0.15, 1.0]),
                 DrawShaded::new("main", "main"),
               ]);
    pipeline.layers = vec![layer];
  }
  
  fn handle_events(&mut self, events: &[WindowEvent], world: &mut World, _: &mut AssetManager, _: &mut Pipeline) -> Trans {
    // Exit if user hits Escape or closes the window
    use amethyst::event::*;
    for event in events {
      match event.payload {
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return Trans::Quit,
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::Right)) => {
          let mut camera = world.write_resource::<Camera>();
          camera.eye[2] += 0.1;
        }
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::Left)) => {
          let mut camera = world.write_resource::<Camera>();
          camera.eye[2] -= 0.1;
        }
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::Down)) => {
          let mut camera = world.write_resource::<Camera>();
          camera.eye[0] -= 0.1;
        }
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::Up)) => {
          let mut camera = world.write_resource::<Camera>();
          camera.eye[0] += 0.1;
        }
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::PageUp)) => {
          let mut camera = world.write_resource::<Camera>();
          camera.eye[1] += 0.1;
        }
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::PageDown)) => {
          let mut camera = world.write_resource::<Camera>();
          camera.eye[1] -= 0.1;
        }
        Event::Closed => return Trans::Quit,
        _ => (),
      }
    }
    Trans::None
  }
  
  fn update(&mut self, world: &mut World, _: &mut AssetManager, _: &mut Pipeline) -> Trans {
    use amethyst::world_resources::Time;
    
    let time = world.read_resource::<Time>();
    self.t += time.delta_time.subsec_nanos() as f32 / 1.0e9;
    self.c += 1;
    
    if self.t >= 1. {
      self.fps = self.c as f32;
      println!("FPS: {}", self.fps);
      
      self.c = 0;
      self.t -= 1.;
    }
    
    Trans::None
  }
}
