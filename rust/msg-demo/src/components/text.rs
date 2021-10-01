use super::StatelessComponent;

pub struct Text {
    pub text: String,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            text: String::default(),
        }
    }
}

impl StatelessComponent for Text {
    fn greet(&self) -> &'static str {
        "Hi"
    }
}
