use animator::AnimatorPlugin;
use asset_loader::AssetLoaderPlugin;
use bevy::prelude::{App, ClearColor, Color};
use bevy::DefaultPlugins;
use bevy::window::WindowDescriptor;
use cursor::CursorPlugin;
use gameplay::GameplayPlugin;
use kayak_ui::bevy::BevyKayakUIPlugin;
use ui::UiPlugin;
use map::MapPlugin;
use turn::TurnPlugin;

mod map;
mod turn;
mod cursor;
mod gameplay;
mod animator;
mod asset_loader;
mod ui;

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
      title: "Tic Tac Toe".to_string(),
      width: 612.0,
      height: 612.0,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
    .add_plugins(DefaultPlugins)
    .add_plugin(BevyKayakUIPlugin)
    .add_plugin(AssetLoaderPlugin)
    .add_plugin(UiPlugin)
    .add_plugin(MapPlugin)
    .add_plugin(TurnPlugin)
    .add_plugin(GameplayPlugin)
    .add_plugin(CursorPlugin)
    .add_plugin(AnimatorPlugin)
    .run();
} 