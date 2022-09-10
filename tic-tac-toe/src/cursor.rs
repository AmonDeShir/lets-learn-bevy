use std::collections::LinkedList;

use bevy::prelude::{Plugin, Component, Commands, EventReader, Res, Query, Camera, GlobalTransform, With, Transform, Handle, Image, Vec3, SystemSet};
use bevy::input::mouse::MouseMotion; 
use bevy::window::Windows;
use bevy::render::camera::RenderTarget;
use bevy::sprite::SpriteBundle;

use crate::GameState;
use crate::animator::{ScaleAnimator, AnimationState};
use crate::asset_loader::Textures;
use crate::map::{cursor_to_word, MainCamera};
use crate::turn::TurnChangeEvent;

pub struct CursorPlugin;

#[derive(Component)]
pub struct Cursor;

#[derive(PartialEq)]
pub enum ModeValue {
  Circle,
  Cross,
}

impl ModeValue {
  pub fn from_bool(value: bool) -> ModeValue {
    if value {
      return ModeValue::Cross;
    }

    return ModeValue::Circle;
  }

  pub fn to_texture(&self, textures: &Textures) -> Handle<Image> {
    if *self == ModeValue::Circle {
      return textures.circle.clone();
    }

    return textures.cross.clone()
  }
}


#[derive(Component)]
pub struct Mode(ModeValue);

impl Plugin for CursorPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_startup_system(init_cursor);
    app.add_system(handle_move);
    app.add_system(handle_turn_change);
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(prepare_cursor));
    app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(close_cursor));

  }
}

pub fn init_cursor(mut commands: Commands, textures: Res<Textures>) {
  commands
    .spawn()
    .insert(Cursor)
    .insert(Mode(ModeValue::Circle))
    .insert(ScaleAnimator(LinkedList::new()))
    .insert_bundle(SpriteBundle {
      texture: textures.circle.clone(),
      transform: Transform {
        translation: Vec3::new(0.0, 0.0, 2.0),
        scale: Vec3::new(0.0, 0.0, 1.0),
        ..Default::default()
      },
      ..Default::default()
    });
}

pub fn handle_move(
  mut events: EventReader<MouseMotion>, 
  window: Res<Windows>,
  mouse_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut cursor_query: Query<&mut Transform, With<Cursor>>
) {
  let mut position = Option::None;

  //Get cursor position 
  for _ in events.iter() {
    let (camera, camera_transform) = mouse_query.get_single().expect("Gameplay(handle_move system) error: There is more than one main camera!");

    let window = if let RenderTarget::Window(id) = camera.target {
      window.get(id).expect(&format!("Cursor(handle_move system) error: Cannot get target window({})!", id))
    } else {
      window.get_primary().expect("Cursor(handle_move system) error: Cannot get primary window!")
    }; 

    if let Some(screen_pos) = window.cursor_position() {
      position = Option::Some(cursor_to_word(screen_pos, window, camera_transform, camera));
    }
  }

  let position = match position {
    Some(pos) => pos,
    None => return
  };

  let mut transform = cursor_query.get_single_mut().expect("Cursor(handle_move system) error: There is more or less than one cursor!");

  transform.translation.x = position.x.into();
  transform.translation.y = position.y.into();
}

pub fn handle_turn_change(
  mut events: EventReader<TurnChangeEvent>, 
  textures: Res<Textures>, 
  mut query: Query<(&mut Transform, &mut ScaleAnimator, &mut Handle<Image>, &mut Mode), With<Cursor>>
) {
  for TurnChangeEvent(turn) in events.iter() {
    let (mut transform, mut animator, mut image, mut mode) = query.get_single_mut().expect("Cursor(handle_mouse_up system) error: There is more or less than one cursor!");
    
    if ModeValue::from_bool(*turn) != mode.0 {
      mode.0 = ModeValue::from_bool(*turn);
      *image = mode.0.to_texture(&textures);
      transform.scale = Vec3::new(0.0, 0.0, 1.0);
      
      if animator.0.len() > 0 {
        animator.0.clear();
      }

      animator.0.push_back(AnimationState::Request(Vec3::new(0.5, 0.5, 1.0), 0.25));
    }
  }
}

fn close_cursor(mut query: Query<&mut ScaleAnimator, With<Cursor>>) {
  let mut animator = query.get_single_mut().expect("Cursor(reset_turn system) error: There is more or less than one cursor!");
  animator.0.clear();
}

fn prepare_cursor(
  textures: Res<Textures>, 
  mut query: Query<(&mut Transform, &mut ScaleAnimator, &mut Handle<Image>, &mut Mode), With<Cursor>>
) {
  let (mut transform, mut animator, mut image, mut mode) = query.get_single_mut().expect("Cursor(reset_turn system) error: There is more or less than one cursor!");
  
  mode.0 = ModeValue::Circle;
  *image = mode.0.to_texture(&textures);
  transform.scale = Vec3::new(0.0, 0.0, 1.0);
  
  if animator.0.len() > 0 {
    animator.0.clear();
  }

  animator.0.push_back(AnimationState::Request(Vec3::new(0.5, 0.5, 1.0), 0.25));
}