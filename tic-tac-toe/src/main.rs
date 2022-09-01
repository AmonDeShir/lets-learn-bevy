use bevy::{prelude::{App, ClearColor, Color}, DefaultPlugins, window::WindowDescriptor};
use greeter::GreeterPlugin;
use map::MapPlugin;

mod greeter;
mod map;

fn main() {
  App::new()
    .insert_resource(bevy::render::texture::ImageSettings::default_nearest())  
    .insert_resource(WindowDescriptor {
      title: "TicTacToe".to_string(),
      width: 512.0,
      height: 512.0,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
    .add_plugins(DefaultPlugins)
    .add_plugin(GreeterPlugin)
    .add_plugin(MapPlugin)
    .run();
} 