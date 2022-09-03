use bevy::prelude::{Plugin, Commands, AssetServer, Res, ResMut, SystemSet}; 
use kayak_ui::bevy::{UICameraBundle, FontMapping, BevyContext};

use crate::GameState;

use self::main_menu::create_main_menu;

mod main_menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_startup_system(load_assets);

    app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(create_main_menu));
    app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy));
  }
}

fn load_assets(mut commands: Commands, mut font_mapping: ResMut<FontMapping>, asset_server: Res<AssetServer>) {
  commands.spawn_bundle(UICameraBundle::new());
  font_mapping.set_default(asset_server.load("backto1982.kayak_font"));
}

fn destroy(mut commands: Commands) {
  commands.remove_resource::<BevyContext>();
}