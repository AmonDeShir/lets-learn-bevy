use bevy::prelude::{Plugin, Component, Commands, Res, ResMut, Query, Transform, With, SystemSet};
use kayak_ui::core::{Binding, MutableBound};

use crate::{player::{Player, PLAYER_WIDTH}, GameState};

pub struct PointsPlugin;

impl Plugin for PointsPlugin { 
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(update_points));
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(reset));
  }
}

#[derive(Component, Clone, PartialEq)]
pub struct Points(pub i32);

fn reset(mut commands: Commands) {
  commands.insert_resource(Points(0));
  commands.insert_resource(Binding::<Points>::new(Points(0)));
}

fn update_points(mut points: ResMut<Points>, binding: Res<Binding<Points>>, query: Query<&Transform, With<Player>>) {  
  for transform in query.iter() {    
    points.0 = (transform.translation.x / PLAYER_WIDTH).floor() as i32;
    binding.set(points.clone());
  }
}