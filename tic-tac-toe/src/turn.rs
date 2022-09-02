use bevy::prelude::Plugin;

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
    app.init_resource::<Turn>();
    app.add_event::<TurnChangeEvent>();
  }
}