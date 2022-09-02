use bevy::prelude::{Plugin, Commands, AssetServer, Res, Handle, Image, StartupStage};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
  }
}

pub struct Textures {
  pub border: Handle<Image>,
  pub tile: Handle<Image>,
  pub circle: Handle<Image>,
  pub cross: Handle<Image>,
}

fn load_assets(
  mut commands: Commands, 
  assets: Res<AssetServer>, 
) {
  commands.insert_resource(Textures {
    border: assets.load("border.png"),
    tile: assets.load("tile.png"),
    circle: assets.load("circle.png"),
    cross: assets.load("cross.png"),
  });
}
