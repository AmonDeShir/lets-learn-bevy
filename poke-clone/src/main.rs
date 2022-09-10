use bevy::prelude::{App, ClearColor, Color};
use bevy::{DefaultPlugins, window::WindowDescriptor};
use ui::UiPlugin;

mod ui;
mod poketclone;
mod poketclone_type;
mod poketclone_attack;
mod loading;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
  Loading,
  MainMenu,
  Map,
  Fight,
}

fn main() {
  App::new()
    .add_state(GameState::Loading)
    .insert_resource(bevy::render::texture::ImageSettings::default_nearest())  
    .insert_resource(WindowDescriptor {
      title: "Poke Clone".to_string(),
      width: 612.0,
      height: 612.0,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
    .add_plugins(DefaultPlugins)
    .add_plugin(UiPlugin)
    .run();
} 