use msg_demo::{components::text::Text, Component, Event, View};

#[derive(Debug)]
pub enum Msg {
    TextSubmit,
    TextBlurred,
    TextActive,
    TextInput(String),
}

const MY_TEXT: &'static str = "MY_TEXT";

fn main() {
    // Prepare view
    let mut view = View::<Msg>::default();
    view.mount(MY_TEXT, Box::new(Text::default()));

    println!("ACTIVE => {:?}", view.on(MY_TEXT, Event::Active));
}

impl Component<Msg> for Text {
    fn on(&mut self, ev: Event) -> Msg {
        match ev {
            Event::Active => Msg::TextActive,
            Event::Blur => Msg::TextBlurred,
            Event::Click => Msg::TextActive,
            Event::Key(ch) => {
                self.text.push(ch);
                Msg::TextInput(self.text.clone())
            }
            Event::Submit => Msg::TextSubmit,
        }
    }
}
