use bevy::{prelude::App, DefaultPlugins, window::WindowDescriptor};
use gameplay::GameplayPlugin;
use map::MapPlugin;

mod map;
mod gameplay;

fn main() {
  App::new()
    .insert_resource(bevy::render::texture::ImageSettings::default_nearest())  
    .insert_resource(WindowDescriptor {
      title: "Tic Tac Toe".to_string(),
      width: 512.0,
      height: 512.0,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(MapPlugin)
    .add_plugin(GameplayPlugin)
    .run();
} 