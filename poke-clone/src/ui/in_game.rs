use bevy::prelude::Commands;
use kayak_ui::bevy::BevyContext;
use kayak_ui::core::{rsx, render, widget, Color};
use kayak_ui::widgets::{App, Text};
use kayak_ui::core::styles::{StyleProp, Units, Style};

#[widget]
fn InGameUI() {
  let text_styles = Style {
    left: StyleProp::Value(Units::Pixels(15.0)),
    top: StyleProp::Value(Units::Pixels(15.0)),    
    color: StyleProp::Value(Color::WHITE),
    ..Default::default()
  };

  rsx! {
    <>
      <Text 
        styles={Some(text_styles)} 
        size={30.0} 
        content={format!("UI")}
      />
    </>
  }
}

pub fn create_in_game_ui(mut commands: Commands) {
  let context = BevyContext::new(|context| {
    render! {
      <App>
        <InGameUI />
      </App>
    }
  });

  commands.insert_resource(context);
}
