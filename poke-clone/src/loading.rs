use bevy::prelude::{Plugin, Res, Commands};
use kayak_ui::core::Binding;

use crate::GameState;

pub struct LoadingPlugin;

pub const STAGES: i16 = 6;

/**
  Loading stages:
  1. load data files (poketclone.yaml, attacks.yaml, items.yaml)
  2. load textures (items, poketclones)
  4. load audio
  5. load map definitions
  6. load tiles
*/
#[derive(Clone, PartialEq)]
pub struct LoadingProgress {
  pub stage_max: i32,
  pub stage_value: i32,
  pub msg: String,
  pub max: i16,
  pub value: i16,
}

impl Plugin for LoadingPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_startup_system(prepare);
  }
}

pub fn prepare(mut commands: Commands) {
  let pogress = LoadingProgress {
    max: STAGES,
    value: 1,
    msg: "Loading: pocketclone.yaml".to_string(),
    stage_max: 3,
    stage_value: 0,
  };

  commands.insert_resource(pogress.clone());
  commands.insert_resource(Binding::<LoadingProgress>::new(pogress));

}

pub fn load_pokemon_data() {
  
}