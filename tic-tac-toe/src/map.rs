use bevy::prelude::{Plugin, Commands, Res, AssetServer, Component, Transform, Vec3};
use bevy::sprite::{SpriteBundle};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

pub struct MapPlugin;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Position(i16, i16);

impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_plugin(PixelCameraPlugin);
    app.add_startup_system(add_camera);
    app.add_startup_system(setup_grid);
    app.add_startup_system(create_map_border);
  }
}

fn add_camera(mut commands: Commands) {
  commands.spawn_bundle(PixelCameraBundle::from_resolution(98, 98));
}

fn setup_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
  for x in -1..2 {
    for y in -1..2 {
      commands
        .spawn()
        .insert(Tile)
        .insert(Position(x + 1, y + 1))
        .insert_bundle(SpriteBundle {
          texture: asset_server.load("tile.png"),
          transform: Transform { 
            translation: Vec3 { 
              x: (x * 32).into(), 
              y: (y * 32).into(), 
              z: 1.0 
            },
            ..Default::default()
          },
          ..Default::default()
        });
    }
  }
}

fn create_map_border(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(SpriteBundle {
      texture: asset_server.load("border.png"),
      transform: Transform { 
        translation: Vec3 { 
          x: 0.0, 
          y: 0.0, 
          z: 0.0 
        },
        ..Default::default()
      },
      ..Default::default()
    });
}