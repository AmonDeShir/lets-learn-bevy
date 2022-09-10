use std::collections::LinkedList;
use std::f32::consts::PI;

use bevy::prelude::{Component, Plugin, Commands, Res, SpriteBundle, Visibility, Transform, Vec3, With, Query, SystemSet, EventReader, Handle, Image, SpatialBundle, Without, BuildChildren, Quat}; 
use bevy::time::Time;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;

use crate::loader::GameTextures; 
use crate::GameState;
use crate::animator::{RotationAnimator, AnimationState};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component)]
pub struct Jump(JumpPhase, f32);

#[derive(PartialEq)]
pub enum JumpPhase {
  RAISING,
  FLOPPING,
  DONE,
}

#[derive(Component)]
pub struct FlyAnimation(i32, f32);

pub const GRAVITY: f32 = 150.0;
pub const SPEED: f32 = 80.0;
pub const JUMP_ACCELERATION: f32 = 1200.0;
pub const JUMP_RETARDATION: f32 = 1800.0;
pub const MAX_JUMP_SPEED: f32 = 400.0;
pub const PLAYER_WIDTH: f32 = 34.0;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_startup_system(add_player);
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(show));
    app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(hide));  
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(gravity));
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(player_move));
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(jump));
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(handle_input));
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(texture_animation));
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(rotation_animation));
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(reset_player));
  }
}

pub fn add_player(mut commands: Commands, textures: Res<GameTextures>) {
  commands
    .spawn()
    .insert(Player)
    .insert(Jump(JumpPhase::DONE, 0.0))
    .insert(FlyAnimation(0, 0.0))
    .insert(RotationAnimator(LinkedList::new()))
    .insert_bundle(SpatialBundle {
      visibility: Visibility { is_visible: false },
      transform: Transform { 
        translation: Vec3 { 
          x: -17.0, 
          y: -12.0, 
          z: 2.0 
        },
        ..Default::default()
      },
      ..Default::default()
    })
    .with_children(|parent| {
      parent
        .spawn()
        .insert(PlayerSprite)
        .insert_bundle(SpriteBundle {
          texture: textures.bird[0].clone(),
          transform: Transform {
            translation: Vec3 {
              x: 0.0,
              y: 0.0,
              z: 1.0,
            },
            ..Default::default()
          },
          ..Default::default()
        });
    });
    
}

pub fn show(mut query: Query<&mut Visibility, With<Player>>) {
  for mut item in query.iter_mut() {
    item.is_visible = true;
  }
}

pub fn reset_player(mut query: Query<(&mut Transform, &mut Jump, &mut FlyAnimation, &mut RotationAnimator), With<Player>>) {
  for (mut transform, mut jump, mut fly, mut animator) in query.iter_mut() {
    transform.translation.x = 0.0;
    transform.translation.y = 0.0;

    transform.rotation = Quat::IDENTITY;

    jump.0 = JumpPhase::DONE;
    jump.1 = 0.0;

    fly.0 = 0;
    fly.1 = 0.0;

    animator.0.clear();
  }
}

pub fn hide(mut query: Query<&mut Visibility, With<Player>>) {
  for mut item in query.iter_mut() {
    item.is_visible = false;
  }
}

pub fn gravity(time: Res<Time>, mut query: Query<&mut Transform, With<Player>>) {
  for mut player in query.iter_mut() {
    player.translation.y -= GRAVITY * time.delta_seconds();
  }
}

pub fn player_move(time: Res<Time>, mut query: Query<&mut Transform, With<Player>>) {
  for mut player in query.iter_mut() {
    player.translation.x += SPEED * time.delta_seconds();
  }
}

pub fn jump(time: Res<Time>, mut query: Query<(&mut Transform, &mut Jump)>) {
  for (mut transform, mut jump) in query.iter_mut() {
    match jump.0 {
      JumpPhase::DONE => {},
      JumpPhase::RAISING => {
        transform.translation.y += jump.1 * time.delta_seconds() + (JUMP_ACCELERATION * time.delta_seconds().powi(2)) / 2.0;
        jump.1 += JUMP_ACCELERATION * time.delta_seconds();

        if jump.1 >= MAX_JUMP_SPEED {
          jump.0 = JumpPhase::FLOPPING;
        }
      },
      JumpPhase::FLOPPING => {
        transform.translation.y += jump.1 * time.delta_seconds() - (JUMP_RETARDATION * time.delta_seconds().powi(2)) / 2.0;
        jump.1 += -JUMP_RETARDATION * time.delta_seconds();

        if jump.1 <= 0.0 {
          jump.0 = JumpPhase::DONE;
        }
      },
    }
  }
}

pub fn handle_input(mut events: EventReader<KeyboardInput>, mut query: Query<&mut Jump, With<Player>>) {
  for ev in events.iter() {
    match ev.state {
      ButtonState::Released => {
        for mut player in query.iter_mut() {
          if player.0 != JumpPhase::DONE {
            player.1 /= 1.5;
          }

          player.0 = JumpPhase::RAISING;
        }
      },
      _ => {},
    }
  }
}

pub fn texture_animation(
  time: Res<Time>, 
  textures: Res<GameTextures>, 
  mut query_fly: Query<&mut FlyAnimation, (With<Player>, Without<PlayerSprite>)>, 
  mut query_sprite: Query<&mut Handle<Image>, (With<PlayerSprite>, Without<Player>)>
) {  
  let mut animation = query_fly.get_single_mut().expect("Player, texture_animation system), Error: There is more or less than one PlayerSprite Component");
  let mut image = query_sprite.get_single_mut().expect("(Player, image system), Error: player doesn't have sprite entity!");
  
  animation.1 += time.delta_seconds();
    
  if animation.1 > 0.25 {
    animation.0 += 1;
    animation.1 = 0.0;
  }

  if animation.0 > 3 {
    animation.0 = 0;
  }

  if animation.0 == 3 {
    *image = textures.bird[1].clone();
  }
  else {
    *image = textures.bird[animation.0 as usize].clone();
  }
}

pub fn rotation_animation(mut query: Query<(&mut RotationAnimator, &Jump)>) {  
  for (mut animator, jump) in query.iter_mut() {
    if animator.0.len() > 0 {
      continue;
    }
    
    if jump.1 > 0.0 {
      let mut target = Transform { ..Default::default() };
      target.rotate_z(PI / 5.0);

      animator.0.push_back(AnimationState::Request(target.rotation, 0.2));
    }
    else {
      let mut target = Transform { ..Default::default() };
      target.rotate_z(-PI / 2.0);

      animator.0.push_back(AnimationState::Request(target.rotation, 0.1)); 
    }
  }
}