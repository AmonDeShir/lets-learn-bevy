use bevy::prelude::{Handle, Image};
use serde::Serialize;

use crate::{poketclone_type::PoketCloneType, poketclone_attack::Attack};

pub struct PoketClone {
  pub id: String,
  pub name: String,
  pub types: Vec<PoketCloneType>,
  pub img: Handle<Image>,
  pub attacks: Vec<Attack>,
  pub evolution: Option<PoketCloneEvolution>,
  pub stats: PoketCloneStatistics,
}

impl PoketClone {
  pub fn create(data: PoketCloneSerialized, img: Handle<Image>) -> PoketClone {
    PoketClone {
      id: data.id,
      name: data.name,
      types: data.types,
      attacks: data.attacks,
      evolution: data.evolution,
      stats: data.stats,
      img,
    }
  }
}

#[derive(Serialize)]
pub struct PoketCloneSerialized {
  pub id: String,
  pub name: String,
  pub types: Vec<PoketCloneType>,
  pub img: String,
  pub attacks: Vec<Attack>,
  pub evolution: Option<PoketCloneEvolution>,
  pub stats: PoketCloneStatistics,
}

#[derive(Serialize)]
pub struct PoketCloneEvolution {
  pub to: String,
  pub level: i16,
}

#[derive(Serialize)]
pub struct PoketCloneStatistics {
  pub hp: i16,
  pub attack: i16,
  pub defense: i16,
  pub special_attack: i16,
  pub special_defence: i16,
  pub speed: i16,
}