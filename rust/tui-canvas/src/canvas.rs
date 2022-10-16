use tui_realm_stdlib::Canvas as TuiCanvas;
use tuirealm::props::Shape;
use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent, NoUserEvent,
};

use super::Msg;

#[derive(MockComponent)]
pub struct Canvas {
    component: TuiCanvas,
}

impl Canvas {
    pub fn new(shapes: &[Shape], width: f64, height: f64) -> Self {
        Self {
            component: TuiCanvas::default()
                .data(shapes)
                .x_bounds((0.0, width))
                .y_bounds((0.0, height)),
        }
    }
}

impl Component<Msg, NoUserEvent> for Canvas {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::Quit),
            _ => None,
        }
    }
}
