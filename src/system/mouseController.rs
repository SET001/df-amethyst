use amethyst::{
  core::{EventChannel, Transform},
  ecs::{ParallelRunnable, System, SystemBuilder},
  input::{get_input_axis_simple, InputEvent, InputHandler},
  prelude::IntoQuery,
  shrev::ReaderId,
};

use crate::component::MouseController;

pub struct MouseControllerSystem {
  event_reader: ReaderId<InputEvent>,
}

impl MouseControllerSystem {
  pub fn new(event_reader: ReaderId<InputEvent>) -> Self {
    Self { event_reader }
  }
}

impl System for MouseControllerSystem {
  fn build(mut self) -> Box<dyn ParallelRunnable> {
    Box::new(
      SystemBuilder::new("MouseControllerSystem")
        .write_resource::<EventChannel<InputEvent>>()
        .read_resource::<InputHandler>()
        .with_query(<(&MouseController, &mut Transform)>::query())
        .build(move |_commands, _, (events, input), _| {
          let event_reader = &mut self.event_reader;
          events.read(event_reader).for_each(|event| match event {
            InputEvent::MouseMoved { delta_x, delta_y } => {
              // println!(
              //   "MouseMoved: {:?}:{:?} -> {:?}",
              //   delta_x,
              //   delta_y,
              //   input.mouse_position()
              // );
            }
            InputEvent::MouseButtonPressed(button) => {
              // println!("MouseButtonPressed: {:?}", button);
            }
            _ => (),
          });
        }),
    )
  }
}
