use bevy::{prelude::{Plugin, Commands, Component, Color, Res, Visibility, Transform, Vec3, Query, SystemSet, With, Without}, sprite::SpriteBundle, time::Time};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin, PixelBorderPlugin};

use crate::{loader::GameTextures, GameState, player::SPEED};

#[derive(Component)]
pub struct MapItem;

pub struct MapPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MapBackground;

pub const WIDTH: i32 = 256;
pub const HEIGHT: i32 = 256; 

impl Plugin for MapPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_plugin(PixelCameraPlugin);
    app.add_plugin(PixelBorderPlugin { color: Color::rgb(0.6, 0.6, 0.6)});
    app.add_startup_system(add_camera);
    app.add_startup_system(add_background);
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(camera_move));
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(show_map));
    app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(hide_map));
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(restore_map));    
  }
}

pub fn add_camera(mut commands: Commands) {
  commands.spawn_bundle(PixelCameraBundle::from_resolution(WIDTH, HEIGHT)).insert(MainCamera);
}

pub fn add_background(mut commands: Commands, textures: Res<GameTextures>) {
  commands
    .spawn()
    .insert(MapItem)
    .insert(MapBackground)
    .insert_bundle(SpriteBundle {
      visibility: Visibility { is_visible: false },
      texture: textures.background_day.clone(),
      transform: Transform { 
        translation: Vec3 { 
          x: 0.0, 
          y: 0.0, 
          z: 1.0 
        },
        ..Default::default()
      },
      ..Default::default()
    });
}

pub fn restore_map(
  mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<MapItem>)>,
  mut map_query: Query<&mut Transform, (With<MapItem>, Without<MainCamera>)>,
) {
  for mut transform in camera_query.iter_mut() {
    transform.translation.x = 0.0;
  }

  for mut transform in map_query.iter_mut() {
    transform.translation.x = 0.0;
  }
}

pub fn show_map(mut query: Query<&mut Visibility, With<MapItem>>) {
  for mut item in query.iter_mut() {
    item.is_visible = true;
  }
}

pub fn hide_map(mut query: Query<&mut Visibility, With<MapItem>>) {
  for mut item in query.iter_mut() {
    item.is_visible = false;
  }
}

pub fn camera_move(
  time: Res<Time>, 
  mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<MapBackground>)>,
  mut background_query: Query<&mut Transform, (With<MapBackground>, Without<MainCamera>)>
) {
  for mut camera in camera_query.iter_mut() {
    camera.translation.x += SPEED * time.delta_seconds();
  }

  for mut background in background_query.iter_mut() {
    background.translation.x += SPEED * time.delta_seconds();
  }
}