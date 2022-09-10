use std::f32::consts::PI;

use bevy::{prelude::{Plugin, Component, Commands, Res, With, Query, Transform, Entity, Vec3, SystemSet, ResMut, State, Vec2, BuildChildren, SpatialBundle, Visibility}, sprite::SpriteBundle, time::Time};

use crate::{map::{MainCamera, WIDTH}, loader::GameTextures, GameState, player::{Player, PLAYER_WIDTH}};

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(add_pipes));
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(remove_pipes));
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(collision));
    app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(remove_all));
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(prepare_pipes));
  }
}

#[derive(Component)]
pub struct Pipe {
  pub x: f32,
  pub lower_y: f32,
  pub upper_y: f32,
}

pub const PIPE_WIDTH: f32 = 52.0;
pub const PIPE_HEIGHT: f32 = 320.0;

fn remove_all(mut pipes: Query<&mut Visibility, With<Pipe>>) {
  for mut visability in pipes.iter_mut() {
    visability.is_visible = false;
  }
}

fn prepare_pipes(mut commands: Commands, mut pipes: Query<Entity, With<Pipe>>) {
  for entity in pipes.iter_mut() {
    commands.entity(entity).despawn();
  } 
}

fn remove_pipes(mut commands: Commands, camera_query: Query<&Transform, With<MainCamera>>, pipes: Query<(Entity, &Transform), With<Pipe>>) {
  let camera = camera_query.get_single().expect("(Pipes, add_pipes system) Error: There is more or less than one main camera!");
  let min_pos = (camera.translation.x - (WIDTH / 2) as f32) - 54.0;
  let max_pos = (camera.translation.x + (WIDTH / 2) as f32) - 54.0;

  for (entity, transform) in pipes.iter() {
    if transform.translation.x < min_pos {
      commands.entity(entity).despawn();
    }

    if transform.translation.x > max_pos {
      commands.entity(entity).despawn();
    }
  }
}

fn add_pipes(mut commands: Commands, textures: Res<GameTextures>, camera_query: Query<&Transform, With<MainCamera>>, pipes: Query<&Transform, With<Pipe>>) {
  let pipes_count = (WIDTH / 54) + 1;
  
  let camera = camera_query.get_single().expect("(Pipes, add_pipes system) Error: There is more or less than one main camera!");
  let last = match pipes.iter().last() {
    Some(pipe) => pipe.translation.x,
    _ => 0.0,
  };

  if pipes.iter().len() < pipes_count as usize  {
    let spawn_pos = camera.translation.x + (WIDTH / 2) as f32 + 60.0;

    if spawn_pos - last < 60.0 + 50.0 * rand::random::<f32>() {
      return;
    }
    
    let mut upper_transform = Transform::identity();
    upper_transform.rotate_z(PI);
    upper_transform.translation.y = 200.0 + 50.0 * rand::random::<f32>();
    let upper_pos = upper_transform.translation.y.clone();
 
    let mut lower_transform = Transform::identity();
    lower_transform.translation.y = -200.0 - 50.0 * rand::random::<f32>();
    let lower_pos = lower_transform.translation.y.clone();

    commands
      .spawn()
      .insert(Pipe {
        x: spawn_pos,
        lower_y: lower_pos,
        upper_y: upper_pos,
      })
      .insert_bundle(SpatialBundle {
        transform: Transform { 
          translation: Vec3 {
            x: spawn_pos,
            y: 0.0,
            z: 2.0,
          },
          ..Default::default()
        },
        ..Default::default()
      })
      .with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
          texture: textures.pipe_green.clone(),
          transform: lower_transform,
          ..Default::default()
        });
      })
      .with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
          texture: textures.pipe_green.clone(),
          transform: upper_transform,
          ..Default::default()
        });
      });
  }
}

fn collision(
  mut state: ResMut<State<GameState>>,
  player_query: Query<&Transform, With<Player>>,
  pipes_query: Query<&Pipe, With<Pipe>>
) {
  for player in player_query.iter() {
    for pipe in pipes_query.iter() {
      if collide(player.translation, Vec2 { x: 24.0, y: 24.0}, &pipe) {
        if let Err(_) = state.set(GameState::Result) {};
        continue;
      }
    }
  }
}

fn collide(pos: Vec3, size: Vec2, pipe: &Pipe) -> bool {
  let margin = 10.0;

  let player_min = Vec2 {
    x: (pos.x - 17.0) - PLAYER_WIDTH / 2.0,
    y: (pos.y - 12.0) - PLAYER_WIDTH / 2.0,
  };

  let player_max = Vec2 {
    x: (pos.x - 17.0) + PLAYER_WIDTH / 2.0,
    y: (pos.y - 12.0) + PLAYER_WIDTH / 2.0,
  };

  let pipe_min = Vec2 { 
    x: pipe.x - (PIPE_WIDTH - margin)  / 2.0,
    y: 0.0,
  };

  let pipe_max = Vec2 { 
    x: pipe.x + (PIPE_WIDTH - margin)  / 2.0,
    y: pipe.upper_y - PIPE_HEIGHT / 2.0,
  };

  let pipe_min = Vec2 {
    x: pipe.x - (PIPE_WIDTH - margin) / 2.0,
    y: pipe.lower_y + (PIPE_HEIGHT - 2.0 * margin) / 2.0,
  };
  
  player_max.x > pipe_min.x && 
  player_min.x < pipe_max.x &&
  (player_max.y > pipe_max.y || player_min.y < pipe_min.y)

}