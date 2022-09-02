use bevy::prelude::Plugin;

pub struct TurnPlugin;
pub struct Turn(pub bool);

impl Default for Turn {
  fn default() -> Self {
    Turn(false)
  }
}

impl Plugin for TurnPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.init_resource::<Turn>();
  }
}