use bevy::prelude::{App, ClearColor, Color};
use bevy::{DefaultPlugins, window::WindowDescriptor};
use loader::AssetLoaderPlugin;
use map::MapPlugin;
use pipes::PipesPlugin;
use player::PlayerPlugin;
use points::PointsPlugin;
use ui::UiPlugin;
use animator::AnimatorPlugin;

mod player;
mod loader;
mod ui;
mod map;
mod animator;
mod pipes;
mod points;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
  MainMenu,
  Game,
  Result,
}

fn main() {
  App::new()
    .add_state(GameState::MainMenu)
    .insert_resource(bevy::render::texture::ImageSettings::default_nearest())  
    .insert_resource(WindowDescriptor {
      title: "Flappy Bird".to_string(),
      width: 612.0,
      height: 612.0,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
    .add_plugins(DefaultPlugins)
    .add_plugin(AssetLoaderPlugin)
    .add_plugin(UiPlugin)
    .add_plugin(MapPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(AnimatorPlugin)
    .add_plugin(PipesPlugin)
    .add_plugin(PointsPlugin)
    .run();
} 