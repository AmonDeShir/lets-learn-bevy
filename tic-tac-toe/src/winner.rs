use bevy::{prelude::{Plugin, App, Commands, Query, SystemSet, ResMut, State}};

use crate::gameplay::{Cross, Circle}; 
use crate::map::Position;
use crate::GameState;

pub struct WinnerPlugin;

impl Plugin for WinnerPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(setup);
    app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup));
    app.add_system_set(SystemSet::on_update(GameState::Game).with_system(check_win_condition));
  }
}

#[derive(PartialEq)]
pub enum Winner {
  Circle,
  Cross,
  None
}

impl Winner {
  pub fn to_string(&self) -> String {
    match self {
      Winner::Cross => "Cross".to_string(),
      Winner::Circle => "Circle".to_string(),
      Winner::None => "Nobody".to_string(),
    }
  }
}

fn setup(mut commands: Commands) {
  commands.insert_resource(Winner::None);
}

fn check_win_condition(
  mut commands: Commands,
  mut game_state:ResMut<State<GameState>>,
  crosses: Query<&Cross>,
  circles: Query<&Circle>,
) {
  let moves_circle = circles.iter().map(|data| &data.0).collect::<Vec<&Position>>();
  let moves_cross = crosses.iter().map(|data| &data.0).collect::<Vec<&Position>>();

  if is_winner(moves_array_to_board(&moves_circle)) {
    commands.insert_resource(Winner::Circle);
    if let Ok(_) = game_state.set(GameState::Result) {};
  }
  
  if is_winner(moves_array_to_board(&moves_cross)) {
    commands.insert_resource(Winner::Cross);
    if let Ok(_) = game_state.set(GameState::Result) {};
  }

  if crosses.iter().len() + circles.iter().len() >= 9 {
    commands.insert_resource(Winner::None);
    if let Ok(_) = game_state.set(GameState::Result) {};
  }
}

const EMPTY_MAP: [[bool; 3]; 3] = [
  [false, false, false], 
  [false, false, false], 
  [false, false, false]
];

fn moves_array_to_board(moves: &[&Position]) -> [[bool; 3]; 3] {
  let mut map = EMPTY_MAP;
  
  for pos in moves.iter() {
    if pos.0 > 3 || pos.0 < 0 || pos.1 > 3 || pos.1 < 0 {
      panic!("Winner(moves_array_to_board) error: Incorrect board position! {:?}", pos);
    }
    
    map[pos.1 as usize][pos.0 as usize] = true; 
  }

  return map;
}

fn is_winner(map: [[bool; 3]; 3]) -> bool {
  //Horizontal line
  (map[0][0] && map[0][1] && map[0][2]) ||
  (map[1][0] && map[1][1] && map[1][2]) ||
  (map[2][0] && map[2][1] && map[2][2]) ||
  //Vertical line
  (map[0][0] && map[1][0] && map[2][0]) ||
  (map[0][1] && map[1][1] && map[2][1]) ||
  (map[0][2] && map[1][2] && map[2][2]) ||
  //Diagonal
  (map[0][0] && map[1][1] && map[2][2]) ||
  (map[0][2] && map[1][1] && map[2][0])
}