use bevy::prelude::{State, ResMut, Commands};
use kayak_ui::bevy::BevyContext;
use kayak_ui::core::{rsx, render, widget, OnEvent, EventType};
use kayak_ui::widgets::{App, Text, Background};
use kayak_ui::core::styles::{StyleProp, Units, Style};

use crate::ui::button::Button;
use crate::GameState;

#[widget]
fn MainMenuWidget() {
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

  let on_event = OnEvent::new(move |ctx, event| match event.event_type {
    EventType::MouseDown(..) => {
      ctx.query_world::<ResMut<State<GameState>>, _, ()>(|mut state| {
        if let Ok(_) = state.set(GameState::Game) {};
      });
    },
    _ => {}
  });

  rsx! {
    <>
      <Background styles={Some(container_styles)}>
        <Text 
          styles={Some(text_styles)} 
          size={60.0} 
          content={format!("Flappy Bird").to_string()}
        />

        <Button size={40.0} text={"Play".to_string()} on_event={Some(on_event)} />   

      </Background>
    </>
  }
}

pub fn create_main_menu(mut commands: Commands) {
  let context = BevyContext::new(|context| {
    render! {
      <App>
        <MainMenuWidget />
      </App>
    }
  });

  commands.insert_resource(context);
}
