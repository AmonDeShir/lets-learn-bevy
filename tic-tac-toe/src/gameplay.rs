use std::collections::LinkedList;
use std::time::Duration;

use bevy::prelude::{Plugin, Component, MouseButton, Res, Query, With, Camera, GlobalTransform, EventReader, Transform, Commands, Vec3, EventWriter, SystemSet, Without, Entity, ResMut};
use bevy::time::{Timer, Time};
use bevy::window::Windows; 
use bevy::render::camera::RenderTarget;
use bevy::input::{mouse::MouseButtonInput, ButtonState};
use bevy::sprite::SpriteBundle;

use crate::GameState;
use crate::animator::{TranslationAnimator, AnimationState, ScaleAnimator};
use crate::asset_loader::Textures;
use crate::map::{MainCamera, cursor_to_word, tile_pos_from_cursor, Position, Tile, IsFree};
use crate::turn::{Turn, TurnChangeEvent};
use crate::winner::Winner;

pub struct GameplayPlugin;

#[derive(Component)]
pub struct Circle(pub Position);

#[derive(Component)]
pub struct Cross(pub Position);


impl Circle {
  pub fn new(pos: &Position) -> Circle {
    Circle(Position(pos.0.clone(), pos.1.clone()))
  }  
}

impl Cross {
  pub fn new(pos: &Position) -> Cross {
    Cross(Position(pos.0.clone(), pos.1.clone()))
  }  
}

impl Plugin for GameplayPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(handle_move));
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(start_gameplay));
    app.add_system_set(SystemSet::on_exit(GameState::Game).with_system(clear_gameplay));
  }
}

pub fn handle_move(
  mut commands: Commands, 
  mut events: EventReader<MouseButtonInput>, 
  mut ev_turn: EventWriter<TurnChangeEvent>,
  mut gameplay_delay: ResMut<GameplayDelayTimer>,
  winner: Res<Winner>,
  time: Res<Time>,
  textures: Res<Textures>, 
  turn: Res<Turn>,
  window: Res<Windows>,
  mouse_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut tile_query: Query<(&Position, &mut IsFree), With<Tile>>
) {
  let mut tile_position = Option::None;
  let mut cursor_position = Option::None;

  gameplay_delay.timer.tick(time.delta());

  if !gameplay_delay.timer.finished() {
    return;
  }

  if *winner != Winner::None {
    return;
  }

  //Get tile position 
  for event in events.iter() {
    if event.state == ButtonState::Pressed && event.button == MouseButton::Left {
      let (camera, camera_transform) = mouse_query.get_single().expect("Gameplay(handle_move system) error: There is more than one main camera!");

      let window = if let RenderTarget::Window(id) = camera.target {
        window.get(id).expect(&format!("Gameplay(handle_move system) error: Cannot get target window({})!", id))
      } else {
        window.get_primary().expect("Gameplay(handle_move system) error: Cannot get primary window!")
      }; 

      if let Some(screen_pos) = window.cursor_position() {
        let cursor = cursor_to_word(screen_pos, window, camera_transform, camera);

        cursor_position = Option::Some(cursor.clone());
        tile_position = Option::Some(tile_pos_from_cursor(cursor));
      }
    }
  }

  let position = match tile_position {
    Some(pos) => pos,
    None => return
  };

  let cursor = match cursor_position {
    Some(pos) => pos,
    None => return
  };

  for (tile_position, mut is_free) in tile_query.iter_mut() {
    if tile_position != &position || !is_free.0 {
      continue;
    }

    //Occupy tile.
    is_free.0 = false;

    //Spawn cross or circle
    let mut output = commands.spawn_bundle(SpriteBundle {
      texture: if turn.0 { textures.cross.clone() } else { textures.circle.clone() },
      transform: Transform { 
        translation: Vec3 { 
          x: cursor.x,
          y: cursor.y,
          z: 1.0 
        },
        scale: Vec3::new(0.5, 0.5, 1.0),
        ..Default::default()
      },
      ..Default::default()
    });

    let mut transform_animator = LinkedList::new();
    transform_animator.push_back(AnimationState::Request(
      Vec3 {
        x: ((tile_position.0 - 1) * 32).into(), 
        y: ((tile_position.1 - 1) * 32).into(), 
        z: 1.0 
      }, 
      0.20
    ));

    let mut scale_animator = LinkedList::new();
    scale_animator.push_back(AnimationState::Request(Vec3::new(1.0, 1.0, 1.0), 0.25));

    output.insert(ScaleAnimator(scale_animator));
    output.insert(TranslationAnimator(transform_animator));


    if turn.0 { 
      output.insert(Cross::new(&position));
    } 
    else { 
      output.insert(Circle::new(&position));
    }

    //Change turn
    commands.insert_resource(Turn(!turn.0));
    ev_turn.send(TurnChangeEvent(!turn.0));
  }

  return;
}

#[derive(Component)]
pub struct GameplayDelayTimer {
  pub timer: Timer,
}

fn clear_gameplay(
  mut commands: Commands,
  crosses: Query<Entity, (With<Cross>, Without<Circle>)>,
  circles: Query<Entity, (With<Circle>, Without<Cross>)>,
) {
  for entity in crosses.iter() {
    commands.entity(entity).despawn();
  }

  for entity in circles.iter() {
    commands.entity(entity).despawn();
  }
}

fn start_gameplay(mut commands: Commands) {
  commands.insert_resource(GameplayDelayTimer {
    timer: Timer::new(Duration::from_millis(500), false)
  }); 
}