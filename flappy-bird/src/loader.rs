use bevy::prelude::{Plugin, Commands, AssetServer, Res, StartupStage, Handle, Image};

pub struct AssetLoaderPlugin;

pub struct GameTextures {
  pub numbers: [Handle<Image>; 10],
  pub background_day: Handle<Image>,
  pub background_night: Handle<Image>,
  pub bird: [Handle<Image>; 3],
  pub base: Handle<Image>,
  pub pipe_green: Handle<Image>,
  pub pipe_red: Handle<Image>
}

impl Plugin for AssetLoaderPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
  }
}

fn load_assets(
  mut commands: Commands, 
  assets: Res<AssetServer>
) {
  commands.insert_resource(GameTextures {
    numbers: [
      assets.load("sprites/0.png"),
      assets.load("sprites/1.png"),
      assets.load("sprites/2.png"),
      assets.load("sprites/3.png"),
      assets.load("sprites/4.png"),
      assets.load("sprites/5.png"),
      assets.load("sprites/6.png"),
      assets.load("sprites/7.png"),
      assets.load("sprites/8.png"),
      assets.load("sprites/9.png"),
    ],
    background_day: assets.load("sprites/background-day.png"),
    background_night: assets.load("sprites/background-night.png"),
    base: assets.load("sprites/base.png"),
    pipe_green: assets.load("sprites/pipe-green.png"),
    pipe_red: assets.load("sprites/pipe-red.png"),
    bird: [
      assets.load("sprites/yellowbird-downflap.png"),
      assets.load("sprites/yellowbird-midflap.png"),
      assets.load("sprites/yellowbird-upflap.png"),
    ],
  });
}
