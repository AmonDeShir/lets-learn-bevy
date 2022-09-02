use animator::AnimatorPlugin;
use bevy::{prelude::App, DefaultPlugins, window::WindowDescriptor};
use cursor::CursorPlugin;
use gameplay::GameplayPlugin;
use map::MapPlugin;
use turn::TurnPlugin;

mod map;
mod turn;
mod cursor;
mod gameplay;
mod animator;

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
    .add_plugin(TurnPlugin)
    .add_plugin(GameplayPlugin)
    .add_plugin(CursorPlugin)
    .add_plugin(AnimatorPlugin)
    .run();
} 