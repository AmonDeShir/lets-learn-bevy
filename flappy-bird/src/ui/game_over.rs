use bevy::prelude::{State, ResMut, Commands, Res};
use kayak_ui::bevy::BevyContext;
use kayak_ui::core::{rsx, render, widget, OnEvent, EventType};
use kayak_ui::widgets::{App, Text, Background};
use kayak_ui::core::styles::{StyleProp, Units, Style};

use crate::GameState;
use crate::points::Points;
use crate::ui::button::Button;

#[widget]
fn GameOver() {
  let text_styles = Style {
    left: StyleProp::Value(Units::Stretch(1.0)),
    right: StyleProp::Value(Units::Stretch(1.0)),
    ..Default::default()
  };

  let container_styles = Style {
    width: StyleProp::Value(Units::Percentage(100.0)),
    height: StyleProp::Value(Units::Percentage(100.0)),
    max_height: StyleProp::Value(Units::Pixels(400.0)),
    bottom: StyleProp::Value(Units::Stretch(1.0)),
    left: StyleProp::Value(Units::Stretch(0.1)),
    right: StyleProp::Value(Units::Stretch(0.1)),
    top: StyleProp::Value(Units::Stretch(1.0)),
    ..Default::default()
  };

  let buttons_container_styles = Style {
    width: StyleProp::Value(Units::Percentage(100.0)),
    height: StyleProp::Value(Units::Percentage(100.0)),
    max_height: StyleProp::Value(Units::Pixels(200.0)),
    bottom: StyleProp::Value(Units::Stretch(1.0)),
    left: StyleProp::Value(Units::Stretch(0.1)),
    right: StyleProp::Value(Units::Stretch(0.1)),
    top: StyleProp::Value(Units::Stretch(1.0)),
    ..Default::default()
  };

  let replay_btn_event = OnEvent::new(move |ctx, event| match event.event_type {
    EventType::MouseDown(..) => {
      ctx.query_world::<ResMut<State<GameState>>, _, ()>(|mut state| {
        if let Ok(_) = state.set(GameState::Game) {};
      });
    },
    _ => {}
  });

  let main_menu_btn_event = OnEvent::new(move |ctx, event| match event.event_type {
    EventType::MouseDown(..) => {
      ctx.query_world::<ResMut<State<GameState>>, _, ()>(|mut state| {
        if let Ok(_) = state.set(GameState::MainMenu) {};
      });
    },
    _ => {}
  });

  let points = context.query_world::<Res<Points>, _, String>(|points| format!("Score: {}", points.0));

  rsx! {
    <>
      <Background styles={Some(container_styles)}>
        <Text 
          styles={Some(text_styles)} 
          size={60.0} 
          content={"Game Over!".to_string()}
        />

        <Background styles={Some(buttons_container_styles)}>
          <Button size={40.0} text={"Play again".to_string()} on_event={Some(replay_btn_event)} />   
          <Button size={40.0} text={"Main menu".to_string()} on_event={Some(main_menu_btn_event)} />   
        </Background>
      </Background>
    </>
  }
}

pub fn create_game_over_screen(mut commands: Commands) {
  let context = BevyContext::new(|context| {
    render! {
      <App>
        <GameOver />
      </App>
    }
  });

  commands.insert_resource(context);
}
