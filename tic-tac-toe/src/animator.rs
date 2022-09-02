use std::collections::LinkedList;
use bevy::{prelude::{Component, Transform, Plugin, Query, Res, Vec3}, time::Time};

pub struct AnimatorPlugin;

#[derive(Debug)]
pub enum AnimationState<T, S> {
  Request(T, f32),
  Animate(T, f32, S),
}

#[derive(Component)]
pub struct ScaleAnimator(pub LinkedList<AnimationState<Vec3, Vec3>>);

#[derive(Component)]
pub struct TranslationAnimator(pub LinkedList<AnimationState<Vec3, Vec3>>);

impl Plugin for AnimatorPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_system(animate_scale);
    app.add_system(animate_translation);
  }
}

pub fn animate_scale(
  time: Res<Time>,
  mut query: Query<(&mut ScaleAnimator, &mut Transform)>
) {
  for (mut animation_list, mut transform) in query.iter_mut() {
    let animation = match animation_list.0.front_mut() {
      Some(data) => data,
      None => continue
    };

    match animation {
      AnimationState::Request(scale, time) => {
        let speed = (*scale - transform.scale) / *time;
        *animation = AnimationState::Animate(*scale, *time, speed);
      },
      AnimationState::Animate(scale, remaining_time, speed) => {
        if *remaining_time <= 0.0 {
          animation_list.0.pop_front();
          continue;
        }

        if *remaining_time - time.delta_seconds() <= 0.0 {
          transform.scale.x = scale.x;
          transform.scale.y = scale.y;
          transform.scale.z = scale.z;
        }
        else {
          transform.scale.x += speed.x * time.delta_seconds();
          transform.scale.y += speed.y * time.delta_seconds();
          transform.scale.z += speed.z * time.delta_seconds();
        }
        
        *animation = AnimationState::Animate(*scale, *remaining_time - time.delta_seconds(), *speed);
      }
    }
  }
}

pub fn animate_translation(
  time: Res<Time>,
  mut query: Query<(&mut TranslationAnimator, &mut Transform)>
) {
  for (mut animation_list, mut transform) in query.iter_mut() {
    let animation = match animation_list.0.front_mut() {
      Some(data) => data,
      None => continue
    };

    match animation {
      AnimationState::Request(translation, time) => {
        let speed = (*translation - transform.translation) / *time;
        *animation = AnimationState::Animate(*translation, *time, speed);

      },
      AnimationState::Animate(translation, remaining_time, speed) => {
        if *remaining_time <= 0.0 {
          animation_list.0.pop_front();

          continue;
        }

        if *remaining_time - time.delta_seconds() <= 0.0 {
          transform.translation.x = translation.x;
          transform.translation.y = translation.y;
          transform.translation.z = translation.z;
        }
        else {
          transform.translation.x += speed.x * time.delta_seconds();
          transform.translation.y += speed.y * time.delta_seconds();
          transform.translation.z += speed.z * time.delta_seconds();
        }
        
        *animation = AnimationState::Animate(*translation, *remaining_time - time.delta_seconds(), *speed);
      }
    }
  }
}