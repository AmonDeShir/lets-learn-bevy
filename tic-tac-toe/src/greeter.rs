use bevy::{prelude::{Plugin, Component, Commands, Query, With, Res, ResMut}, time::{Time, Timer}};

pub struct GreeterPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Title(String);

fn hello_word() {
  println!("Hello world!");
}

fn add_people(mut commands: Commands) {
  commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string())).insert(Title("Lady".to_string()));
  commands.spawn().insert(Person).insert(Name("Rezo Hume".to_string())).insert(Title("Lord".to_string()));
  commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string())).insert(Title("Lady".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Name, &Title), With<Person>>) {
  if timer.0.tick(time.delta()).just_finished() {
    for (Name(name), Title(title)) in query.iter() {
      println!("Greetings {} {}!", title, name);
    }
  }
} 

impl Plugin for GreeterPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)));
    app.add_startup_system(add_people);
    app.add_startup_system(hello_word);
    app.add_system(greet_people);
  }
}