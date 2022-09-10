use bevy::prelude::{State, ResMut, Res, Commands};
use kayak_ui::bevy::{BevyContext, ImageManager};
use kayak_ui::core::{rsx, render, widget, OnEvent, EventType, WidgetProps};
use kayak_ui::widgets::{App, Background, Image};
use kayak_ui::core::styles::{StyleProp, Units, Style};

use crate::GameState;
use crate::ui::button::Button;

use super::LoadingAssets;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct MainMenuProps {
  logo: u16
}

impl WidgetProps for MainMenuProps {
  fn get_on_event(&self) -> Option<OnEvent> { None }
  fn get_children(&self) -> Option<kayak_ui::core::Children> { None }
  fn get_focusable(&self) -> Option<bool> { None }
  fn get_on_layout(&self) -> Option<kayak_ui::core::OnLayout> { None }
  fn get_styles(&self) -> Option<Style> { None }
  fn set_children(&mut self, _: Option<kayak_ui::core::Children>) {}
}

#[widget]
fn MainMenuWidget(props: MainMenuProps) {
  let image_styles = Style {
    left: StyleProp::Value(Units::Stretch(0.8)),
    right: StyleProp::Value(Units::Stretch(0.8)),
    width: StyleProp::Value(Units::Pixels(596.0)),
    height: StyleProp::Value(Units::Pixels(230.0)),
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

  let play_btn_event = OnEvent::new(move |ctx, event| match event.event_type {
    EventType::MouseDown(..) => {
      ctx.query_world::<ResMut<State<GameState>>, _, ()>(|mut state| {
        if let Ok(_) = state.set(GameState::Map) {};
      });
    },
    _ => {}
  });

  let multiplayer_btn_event = OnEvent::new(move |ctx, event| match event.event_type {
    EventType::MouseDown(..) => {
      ctx.query_world::<ResMut<State<GameState>>, _, ()>(|mut state| {
        if let Ok(_) = state.set(GameState::Map) {};
      });
    },
    _ => {}
  });
  
  rsx! {
    <>
      <Background styles={Some(container_styles)}>
        <Image styles={Some(image_styles)} handle={props.logo} />

        <Background styles={Some(buttons_container_styles)}>
          <Button size={40.0} text={"Play".to_string()} on_event={Some(play_btn_event)} />   
          <Button size={40.0} text={"Multiplayer".to_string()} on_event={Some(multiplayer_btn_event)} />   
        </Background>
      </Background>
    </>
  }
}

pub fn create_main_menu_screen(mut commands: Commands, mut image_manager: ResMut<ImageManager>, images: Res<LoadingAssets>) {
  let logo_handle = image_manager.get(&images.logo);

  let context = BevyContext::new(|context| {
    render! {
      <App>
        <MainMenuWidget logo={logo_handle} />
      </App>
    }
  });

  commands.insert_resource(context);
}
