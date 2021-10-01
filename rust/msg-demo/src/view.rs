use crate::{Component, Event};

use std::collections::HashMap;

pub type WrappedComponent<Msg> = Box<dyn Component<Msg>>;

pub struct View<Msg> {
    components: HashMap<&'static str, WrappedComponent<Msg>>,
}

impl<Msg> Default for View<Msg> {
    fn default() -> Self {
        Self {
            components: HashMap::default(),
        }
    }
}

impl<Msg> View<Msg> {
    pub fn mount(&mut self, id: &'static str, component: WrappedComponent<Msg>) {
        self.components.insert(id, component);
    }

    pub fn greet(&self, id: &'static str) -> Option<&str> {
        match self.components.get(id) {
            None => None,
            Some(c) => Some(c.greet()),
        }
    }

    pub fn on(&mut self, id: &'static str, ev: Event) -> Option<Msg> {
        match self.components.get_mut(id) {
            None => None,
            Some(c) => Some(c.on(ev)),
        }
    }
}
