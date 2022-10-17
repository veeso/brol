use tuirealm::props::Color;
use tuirealm::props::Shape;
use tuirealm::tui::widgets::canvas::Rectangle;

const X_SIZE: f64 = 4.0;
const Y_SIZE: f64 = 6.0;

pub struct Render;

impl Render {
    pub fn render(mut x: f64, mut y: f64, data: &str) -> anyhow::Result<Vec<Shape>> {
        let newline_x = x;
        let mut shapes = Vec::new();
        for line in data.lines() {
            // reset x
            x = newline_x;
            // iter line chars
            for symbol in line.chars() {
                if symbol == '*' {
                    shapes.push(Shape::Rectangle(Rectangle {
                        x,
                        y,
                        width: X_SIZE,
                        height: Y_SIZE,
                        color: Color::Yellow,
                    }));
                }
                // incr x
                x += X_SIZE;
            }
            // incr y
            y -= Y_SIZE;
        }
        Ok(shapes)
    }

    pub fn origin_y(canvas_height: f64) -> f64 {
        (canvas_height * Y_SIZE) - (4.0 * Y_SIZE)
    }
}
