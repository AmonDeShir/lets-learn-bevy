use bevy::prelude::{Commands, Res, ResMut};
use kayak_ui::bevy::{BevyContext, ImageManager};
use kayak_ui::core::{rsx, render, widget, OnEvent, WidgetProps};
use kayak_ui::widgets::{App, Background, Image};
use kayak_ui::core::styles::{StyleProp, Units, Style};

use crate::ui::button::Button;
use crate::GameState;

use super::LoadingAssets;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct LoadingProps {
  logo: u16
}

impl WidgetProps for LoadingProps {
  fn get_on_event(&self) -> Option<OnEvent> { None }
  fn get_children(&self) -> Option<kayak_ui::core::Children> { None }
  fn get_focusable(&self) -> Option<bool> { None }
  fn get_on_layout(&self) -> Option<kayak_ui::core::OnLayout> { None }
  fn get_styles(&self) -> Option<Style> { None }
  fn set_children(&mut self, _: Option<kayak_ui::core::Children>) {}
}

#[widget]
fn Loading(props: LoadingProps) {
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

  rsx! {
    <>
      <Background styles={Some(container_styles)}>
        <Image handle={props.logo} />
      </Background>
    </>
  }
}

pub fn create_loading_screen(mut commands: Commands, mut image_manager: ResMut<ImageManager>, images: Res<LoadingAssets>) {
  let logo_handle = image_manager.get(&images.logo);
  
  let context = BevyContext::new(|context| {
    render! {
      <App>
        <Loading logo={logo_handle} />
      </App>
    }
  });

  commands.insert_resource(context);
}
