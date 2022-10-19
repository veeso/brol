use tuirealm::props::Color;
use tuirealm::props::Shape;
use tuirealm::tui::widgets::canvas::Rectangle;

const X_SIZE_LARGE: f64 = 4.0;
const Y_SIZE_LARGE: f64 = 6.0;
const X_SIZE_MEDIUM: f64 = 3.0;
const Y_SIZE_MEDIUM: f64 = 4.0;
const X_SIZE_SMALL: f64 = 1.0;
const Y_SIZE_SMALL: f64 = 3.0;

pub struct Render {
    x_scale: f64,
    y_scale: f64,
}

impl Render {
    /// Instantiate a new render engine
    pub fn new(width: f64, height: f64) -> Self {
        if (width, height) >= (160.0, 36.0) {
            Self {
                x_scale: X_SIZE_LARGE,
                y_scale: Y_SIZE_LARGE,
            }
        } else if (width, height) >= (96.0, 32.0) {
            Self {
                x_scale: X_SIZE_MEDIUM,
                y_scale: Y_SIZE_MEDIUM,
            }
        } else {
            Self {
                x_scale: X_SIZE_SMALL,
                y_scale: Y_SIZE_SMALL,
            }
        }
    }

    pub fn render(&self, mut x: f64, mut y: f64, data: &str) -> anyhow::Result<Vec<Shape>> {
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
                        width: self.x_scale,
                        height: self.y_scale,
                        color: Color::Yellow,
                    }));
                }
                // incr x
                x += self.x_scale;
            }
            // incr y
            y -= self.y_scale;
        }
        Ok(shapes)
    }

    pub fn origin_y(&self, canvas_height: f64) -> f64 {
        (canvas_height * self.y_scale) - (4.0 * self.y_scale)
    }
}
