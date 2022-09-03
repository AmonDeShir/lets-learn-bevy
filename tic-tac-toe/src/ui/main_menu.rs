use bevy::prelude::{State, ResMut, Commands};
use kayak_ui::bevy::BevyContext;
use kayak_ui::core::{rsx, render, widget, OnEvent, EventType, Color, use_state};
use kayak_ui::widgets::{App, Button, Text, Background};
use kayak_ui::core::styles::{StyleProp, Units, Style};

use crate::GameState;

#[widget]
fn MainMenuWidget() {
  let (hover, set_hover, ..) = use_state!(false);

  let text_styles = Style {
    left: StyleProp::Value(Units::Stretch(1.0)),
    right: StyleProp::Value(Units::Stretch(1.0)),
    ..Default::default()
  };

  let button_color = if hover { StyleProp::Value(Color::WHITE) } else { StyleProp::Value(Color::BLACK) };

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

  let button_styles = Style {
    background_color: StyleProp::Value(Color::TRANSPARENT),
    color: button_color,
    top: StyleProp::Value(Units::Stretch(0.1)),
    bottom: StyleProp::Value(Units::Stretch(0.1)),
    ..Default::default()
  };

  let on_event = OnEvent::new(move |ctx, event| match event.event_type {
    EventType::MouseIn(..) => { set_hover(true); },
    EventType::MouseOut(..) => { set_hover(false); },
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
          content={format!("Tic Tac Toe").to_string()}
        >{}</Text>

        <Button styles={Some(button_styles)} on_event={Some(on_event)}>
          <Text 
            size={40.0} 
            content={format!("Play").to_string()}
          >{}</Text>
        </Button>
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
