use bevy::prelude::{Plugin, Component, MouseButton, Res, Query, With, Camera, GlobalTransform, EventReader, Transform, Commands, AssetServer, Vec3};
use bevy::window::Windows; 
use bevy::render::camera::RenderTarget;
use bevy::input::{mouse::MouseButtonInput, ButtonState};
use bevy::sprite::SpriteBundle;

use crate::map::{MainCamera, cursor_to_word, tile_pos_from_cursor, Position, Tile, IsFree};
use crate::turn::Turn;

pub struct GameplayPlugin;

#[derive(Component)]
pub struct Circle(i16, i16);

#[derive(Component)]
pub struct Cross(i16, i16);

impl Circle {
  pub fn new(pos: &Position) -> Circle {
    Circle(pos.0.clone(), pos.1.clone())
  }  
}

impl Cross {
  pub fn new(pos: &Position) -> Cross {
    Cross(pos.0.clone(), pos.1.clone())
  }  
}


impl Plugin for GameplayPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_system(handle_move);
  }
}

pub fn handle_move(
  mut commands: Commands, 
  mut events: EventReader<MouseButtonInput>, 
  asset_server: Res<AssetServer>, 
  turn: Res<Turn>,
  window: Res<Windows>,
  mouse_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
  mut tile_query: Query<(&Position, &mut IsFree), With<Tile>>
) {
  let mut position = Option::None;
  
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
        position = Option::Some(tile_pos_from_cursor(cursor_to_word(screen_pos, window, camera_transform, camera)));
      }
    }
  }

  let position = match position {
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
      texture: asset_server.load(if turn.0 { "cross.png" } else { "circle.png" }),
      transform: Transform { 
        translation: Vec3 { 
          x: ((tile_position.0 - 1) * 32).into(), 
          y: ((tile_position.1 - 1) * 32).into(), 
          z: 1.0 
        },
        ..Default::default()
      },
      ..Default::default()
    });

    if turn.0 { 
      output.insert(Cross::new(&position));
    } 
    else { 
      output.insert(Circle::new(&position));
    }

    //Change turn
    commands.insert_resource(Turn(!turn.0));
  }

  return;
}

