use bevy::prelude::{Plugin, Commands, Res, Component, Transform, Vec3, Color, Vec2, GlobalTransform, Camera, Visibility, SystemSet, Query, With, Without};
use bevy::sprite::{SpriteBundle};
use bevy::window::Window;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin, PixelBorderPlugin};

use crate::GameState;
use crate::asset_loader::Textures;

pub struct MapPlugin;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Border;

#[derive(Component)]
pub struct IsFree(pub bool);

#[derive(Component)]
pub struct MainCamera;


#[derive(Component, Debug, PartialEq, Eq, Hash)]
pub struct Position(pub i16, pub i16);

impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_plugin(PixelCameraPlugin);
    app.add_plugin(PixelBorderPlugin { color: Color::rgb(0.6, 0.6, 0.6)});
    app.add_startup_system(add_camera);
    app.add_startup_system(setup_grid);
    app.add_startup_system(create_map_border);
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(show_map));
    app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(hide_map));
  }
}

pub fn add_camera(mut commands: Commands) {
  commands.spawn_bundle(PixelCameraBundle::from_resolution(98, 98)).insert(MainCamera);
}

pub fn cursor_to_word(pos: Vec2, window: &Window, camera_transform: &GlobalTransform, camera: &Camera) -> Vec2 {
  let window_size = Vec2::new(window.width(), window.height());

  // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
  let ndc = (pos / window_size) * 2.0 - Vec2::ONE;

  // matrix for undoing the projection and camera transform
  let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

  // use it to convert ndc to world-space coordinates
  let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

  // reduce it to a 2D value
  world_pos.truncate()
}

pub fn setup_grid(mut commands: Commands, textures: Res<Textures>) {
  for x in -1..2 {
    for y in -1..2 {
      commands
        .spawn()
        .insert(Tile)
        .insert(IsFree(true))
        .insert(Position(x + 1, y + 1))
        .insert_bundle(SpriteBundle {
          visibility: Visibility { is_visible: false },
          texture: textures.tile.clone(),
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

pub fn tile_pos_from_cursor(pos: Vec2) -> Position {
  let pos = ((pos + 16.0) / 32.0).floor() + 1.0;
  Position(pos.x as i16, pos.y as i16)
}

pub fn create_map_border(mut commands: Commands, textures: Res<Textures>) {
  commands.spawn_bundle(SpriteBundle {
      visibility: Visibility { is_visible: false },
      texture: textures.border.clone(),
      transform: Transform { 
        translation: Vec3 { 
          x: 0.0, 
          y: 0.0, 
          z: 0.0 
        },
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(Border);
}

pub fn show_map(
  mut tiles: Query<(&mut Visibility, &mut IsFree), (With<Tile>, Without<Border>)>,
  mut border: Query<&mut Visibility, (With<Border>, Without<Tile>)>,
) {
  for (mut visibility, mut is_free) in tiles.iter_mut() {
    visibility.is_visible = true;
    is_free.0 = true;
  }

  for mut visibility in border.iter_mut() {
    visibility.is_visible = true;
  }
}

pub fn hide_map(
  mut tiles: Query<&mut Visibility, (With<Tile>, Without<Border>)>,
  mut border: Query<&mut Visibility, (With<Border>, Without<Tile>)>,
) {
  for mut visibility in tiles.iter_mut() {
    visibility.is_visible = false;
  }

  for mut visibility in border.iter_mut() {
    visibility.is_visible = false;
  }
}