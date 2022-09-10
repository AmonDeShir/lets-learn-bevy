use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PoketCloneType {
  Normal,	
  Fire,
  Fighting,	
  Water,
  Flying,	
  Grass,
  Poison,
  Electric,
  Ground,
  Psychic,
  Rock,	
  Ice,
  Bug,
  Dragon,
  Ghost,	
  Dark,
  Steel,
  Fairy,
}

impl PoketCloneType {
  pub fn create(type_str: &str) -> Option<PoketCloneType> {
    match type_str {
      "normal" => Some(PoketCloneType::Normal),	
      "fire" => Some(PoketCloneType::Fire),
      "fighting" => Some(PoketCloneType::Fighting),
      "water" => Some(PoketCloneType::Water),
      "flying" => Some(PoketCloneType::Flying),	
      "grass" => Some(PoketCloneType::Grass),
      "poison" => Some(PoketCloneType::Poison),
      "electric" => Some(PoketCloneType::Electric),
      "ground" => Some(PoketCloneType::Ground),
      "psychic" => Some(PoketCloneType::Psychic),
      "rock" => Some(PoketCloneType::Rock),
      "ice" => Some(PoketCloneType::Ice),
      "ghost" => Some(PoketCloneType::Ghost),	
      "dark" => Some(PoketCloneType::Dark),
      "steel" => Some(PoketCloneType::Steel),
      "fairy" => Some(PoketCloneType::Fairy),
      _ => None,
    }
  }

  pub fn fight(&self, target: PoketCloneType) -> f32 {
    match self {
      Self::Normal => {
        match target {
          Self::Rock => 0.5,
          Self::Ghost => 0.0,
          Self::Steel => 0.5,
          _ => 1.0,
        }
      },
      Self::Fighting => {
        match target {
          Self::Normal => 2.0,
          Self::Flying => 0.5,
          Self::Poison => 0.5,
          Self::Rock => 2.0,
          Self::Bug => 0.5,
          Self::Ghost => 0.0,
          Self::Steel => 2.0,
          Self::Psychic => 0.5,
          Self::Ice => 2.0,
          Self::Dark => 2.0,
          Self::Fairy => 0.5,
          _ => 1.0,
        }
      },
      Self::Flying => {
        match target {
          Self::Flying => 2.0,
          Self::Rock => 0.5,
          Self::Bug => 2.0,
          Self::Steel => 0.5,
          Self::Grass => 2.0,
          Self::Electric => 0.5,
          _ => 1.0,
        }
      },
      Self::Poison => {        
        match target {
          Self::Poison => 0.5,
          Self::Ground => 0.5,
          Self::Rock => 0.5,
          Self::Ghost => 0.5,
          Self::Steel => 0.0,
          Self::Grass => 2.0,
          Self::Fairy => 2.0,
          _ => 1.0,
        }
      },
      Self::Ground => {
        match target {
          Self::Flying => 0.0,
          Self::Poison => 2.0,
          Self::Rock => 2.0,
          Self::Bug => 0.5,
          Self::Steel => 2.0,
          Self::Fire => 2.0,
          Self::Grass => 0.5,
          Self::Electric => 2.0,
          _ => 1.0,
        }
      },
      Self::Rock => {
        match target {
          Self::Fighting => 0.5,
          Self::Flying => 2.0,
          Self::Ground => 0.5,
          Self::Bug => 2.0,
          Self::Steel => 0.5,
          Self::Fire => 2.0,
          Self::Ice => 2.0,
          _ => 1.0,
        }
      },
      Self::Bug => {
        match target {
          Self::Fighting => 0.5,
          Self::Flying => 0.5,
          Self::Poison => 0.5,
          Self::Ghost => 0.5,
          Self::Steel => 0.5,
          Self::Fire => 0.5,
          Self::Grass => 2.0,
          Self::Psychic => 2.0,
          Self::Dark => 2.0,
          Self::Fairy => 0.5,
          _ => 1.0,
        }
      },
      Self::Ghost => {
        match target {
          Self::Normal => 0.0,
          Self::Ghost => 2.0,
          Self::Psychic => 2.0,
          Self::Dark => 0.5,
          _ => 1.0,
        }
      },
      Self::Steel => {
        match target {
          Self::Rock => 2.0,
          Self::Steel => 0.5,
          Self::Fire => 0.5,
          Self::Water => 0.5,
          Self::Electric => 0.5,
          Self::Ice => 2.0,
          Self::Fairy => 2.0,
          _ => 1.0,
        }
      },
      Self::Fire => {
        match target {
          Self::Rock => 0.5,
          Self::Bug => 2.0,
          Self::Steel => 2.0,
          Self::Fire => 0.5,
          Self::Water => 0.5,
          Self::Grass => 2.0,
          Self::Ice => 2.0,
          Self::Dragon => 0.5,

          _ => 1.0,
        }
      },
      Self::Water => {
        match target {
          Self::Ground => 2.0,
          Self::Rock => 2.0,
          Self::Fire => 2.0,
          Self::Water => 0.5,
          Self::Grass => 0.5,
          Self::Dragon => 0.5,
          _ => 1.0,
        }
      },
      Self::Grass => {
        match target {
          Self::Flying => 0.5,
          Self::Poison => 0.5,
          Self::Ground => 2.0,
          Self::Rock => 2.0,
          Self::Bug => 0.5,
          Self::Steel => 0.5,
          Self::Fire => 0.5,
          Self::Water => 2.0,
          Self::Grass => 0.5,
          Self::Dragon => 0.5,
          _ => 1.0,
        }
      },
      Self::Electric => {
        match target {
          Self::Flying => 2.0,
          Self::Ground => 0.0,
          Self::Water => 2.0,
          Self::Grass => 0.5,
          Self::Electric => 0.5,
          Self::Dragon => 0.5,
          _ => 1.0,
        }
      },
      Self::Psychic => {
        match target {
          Self::Fighting => 2.0,
          Self::Poison => 2.0,
          Self::Steel => 0.5,
          Self::Psychic => 0.5,
          Self::Dark => 0.0,
          _ => 1.0,
        }
      },
      Self::Ice => {
        match target {
          Self::Flying => 2.0,
          Self::Ground => 2.0,
          Self::Steel => 0.5,
          Self::Fire => 0.5,
          Self::Water => 0.5,
          Self::Ice => 0.5,
          Self::Dragon => 2.0,

          _ => 1.0,
        }
      },
      Self::Dragon => {
        match target {
          Self::Steel => 0.5,
          Self::Dragon => 2.0,
          Self::Fairy => 0.0,
          _ => 1.0,
        }
      },
      Self::Dark => {
        match target {
          Self::Fighting => 0.5,
          Self::Ghost => 2.0,
          Self::Psychic => 2.0,
          Self::Dark => 0.5,
          Self::Fairy => 0.5,
          _ => 1.0,
        }
      },
      Self::Fairy => {
        match target {
          Self::Fighting => 2.0,
          Self::Poison => 0.5,
          Self::Steel => 0.5,
          Self::Fire => 0.5,
          Self::Ice => 2.0,
          Self::Dragon => 2.0,
          _ => 1.0,
        }
      },
    }
  }
}