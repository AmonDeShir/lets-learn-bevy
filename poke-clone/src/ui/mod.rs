use bevy::prelude::{Plugin, Commands, AssetServer, Res, ResMut, SystemSet, Handle, Image}; 
use kayak_ui::bevy::{UICameraBundle, FontMapping, BevyContext, BevyKayakUIPlugin};

use crate::GameState;

use self::{loading::create_loading_screen, in_game::create_in_game_ui, main_menu::create_main_menu_screen};

mod loading;
mod button;
mod in_game;
mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_plugin(BevyKayakUIPlugin);
    app.add_startup_system(load_assets);

    app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(create_loading_screen));
    app.add_system_set(SystemSet::on_exit(GameState::Loading).with_system(destroy));

    app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(create_main_menu_screen));
    app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy));

    app.add_system_set(SystemSet::on_enter(GameState::Map).with_system(create_in_game_ui));
    app.add_system_set(SystemSet::on_exit(GameState::Map).with_system(destroy));
  }
}

pub struct LoadingAssets {
  logo: Handle<Image>
}

fn load_assets(mut commands: Commands, mut font_mapping: ResMut<FontMapping>, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(UICameraBundle::new());

  let font = asset_server.load("fonts/pokemon.kayak_font");
  
  commands.insert_resource(LoadingAssets {
    logo: asset_server.load("ui/logo.png"),
  });

  font_mapping.set_default(font);
}

fn destroy(mut commands: Commands) {
  commands.remove_resource::<BevyContext>();
}