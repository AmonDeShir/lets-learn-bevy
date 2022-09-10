use bevy::prelude::{Res, Commands};
use kayak_ui::bevy::BevyContext;
use kayak_ui::core::{rsx, render, widget, Color, Binding, Bound};
use kayak_ui::widgets::{App, Text};
use kayak_ui::core::styles::{StyleProp, Units, Style};

use crate::points::Points;

#[widget]
fn InGameUI() {
  let text_styles = Style {
    left: StyleProp::Value(Units::Pixels(15.0)),
    top: StyleProp::Value(Units::Pixels(15.0)),    
    color: StyleProp::Value(Color::WHITE),
    font: StyleProp::Value("Minecraft".to_string()),
    ..Default::default()
  };

  let points = context.query_world::<Res<Binding<Points>>, _, Binding<Points>>(|points| points.clone());
  context.bind(&points);

  let points = points.get().0;

  rsx! {
    <>
      <Text 
        styles={Some(text_styles)} 
        size={30.0} 
        content={format!("Points: {} ", points)}
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
