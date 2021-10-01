pub mod components;
mod view;

pub use view::View;

pub enum Event {
    Click,
    Blur,
    Active,
    Submit,
    Key(char),
}

pub trait StatelessComponent {
    fn greet(&self) -> &'static str;
}

pub trait Component<Msg>: StatelessComponent {
    fn on(&mut self, ev: Event) -> Msg;
}
