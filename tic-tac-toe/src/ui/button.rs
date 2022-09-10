use kayak_ui::core::{rsx, widget, OnEvent, EventType, Color, use_state, WidgetProps};
use kayak_ui::widgets::{Button as BaseButton, Text};
use kayak_ui::core::styles::{StyleProp, Units, Style};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct ButtonProps {
  pub text: String,
  pub size: f32,
  pub on_event: Option<OnEvent>,
}

impl WidgetProps for ButtonProps {
  fn get_on_event(&self) -> Option<OnEvent> {
    self.on_event.clone()
  }

  fn get_children(&self) -> Option<kayak_ui::core::Children> { None }
  fn get_focusable(&self) -> Option<bool> { None }
  fn get_on_layout(&self) -> Option<kayak_ui::core::OnLayout> { None }
  fn get_styles(&self) -> Option<Style> { None }
  fn set_children(&mut self, _: Option<kayak_ui::core::Children>) {}
}

#[widget]
pub fn Button(props: ButtonProps) {
  let (hover, set_hover, ..) = use_state!(false);

  let button_styles = Style {
    background_color: StyleProp::Value(Color::TRANSPARENT),
    color: if hover { StyleProp::Value(Color::WHITE) } else { StyleProp::Value(Color::BLACK) },
    top: StyleProp::Value(Units::Stretch(0.1)),
    bottom: StyleProp::Value(Units::Stretch(0.1)),
    ..Default::default()
  };

  let props_events = props.get_on_event();

  let events = OnEvent::new(move |ctx, event| {
    match event.event_type {
      EventType::MouseIn(..) => { 
        set_hover(true); 
      },
      EventType::MouseOut(..) => { 
        set_hover(false); 
      },
      _ => {}
    }

    if let Some(call) = &props_events {
      call.try_call(ctx, event);
    }
  });

  let text = props.text.clone();

  rsx! {
    <BaseButton styles={Some(button_styles)} on_event={Some(events)}>
      <Text 
        size={props.size} 
        content={text}
      />
    </BaseButton>
  }
}
