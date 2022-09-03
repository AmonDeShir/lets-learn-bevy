use bevy::prelude::{Plugin, SystemSet, ResMut};

use crate::GameState;

pub struct TurnPlugin;
pub struct Turn(pub bool);

pub struct TurnChangeEvent(pub bool);

impl Default for Turn {
  fn default() -> Self {
    Turn(false)
  }
}

impl Plugin for TurnPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .init_resource::<Turn>()
      .add_event::<TurnChangeEvent>()
      .add_system_set(SystemSet::on_enter(GameState::Game).with_system(reset_turn));
  }
}

fn reset_turn(mut turn: ResMut<Turn>) {
  turn.0 = false;
}