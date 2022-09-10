use serde::Serialize;

#[derive(Serialize)]
pub enum AttackType {
  Physical,
  Special,
  Status,
}

#[derive(Serialize)]
pub struct Attack {

}