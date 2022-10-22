//! # Render
//!
//! This module exposes the render engine

use tuirealm::props::Color;
use tuirealm::props::Shape;
use tuirealm::tui::widgets::canvas::{Line, Rectangle};

// Viewports
const X_SIZE_LARGE: f64 = 4.0;
const Y_SIZE_LARGE: f64 = 6.0;
const X_SIZE_MEDIUM: f64 = 3.0;
const Y_SIZE_MEDIUM: f64 = 4.0;
const X_SIZE_SMALL: f64 = 1.0;
const Y_SIZE_SMALL: f64 = 3.0;

/// Render room type.
/// Back way is not rendered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Room {
    /// One exit (front)
    Corridor,
    CorridorWithMazeExit,
    TwoExit,
    TwoExitWithMazeExit,
    ThreeExit,
}

/// Render engine
pub struct Render {
    x_scale: f64,
    y_scale: f64,
    width: f64,
    height: f64,
}

impl Render {
    /// Instantiate a new render engine
    pub fn new(mut width: f64, mut height: f64) -> Self {
        width -= 2.0;
        height -= 2.0;
        if (width, height) >= (160.0, 36.0) {
            Self {
                x_scale: X_SIZE_LARGE,
                y_scale: Y_SIZE_LARGE,
                width,
                height,
            }
        } else if (width, height) >= (96.0, 32.0) {
            Self {
                x_scale: X_SIZE_MEDIUM,
                y_scale: Y_SIZE_MEDIUM,
                width,
                height,
            }
        } else {
            Self {
                x_scale: X_SIZE_SMALL,
                y_scale: Y_SIZE_SMALL,
                width,
                height,
            }
        }
    }

    /// Render shape from ascii
    pub fn ascii_art(&self, mut x: f64, mut y: f64, data: &str, color: Color) -> Vec<Shape> {
        let newline_x = x;
        let mut shapes = Vec::new();
        for line in data.lines().rev() {
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
                        color,
                    }));
                }
                // incr x
                x += self.x_scale;
            }
            // incr y
            y += self.y_scale;
        }
        shapes
    }

    /// Render room
    pub fn render_room(&self, room: Room) -> Vec<Shape> {
        match room {
            Room::Corridor => self.render_room_corridor(),
            Room::CorridorWithMazeExit => self.render_room_corridor_with_maze_exit(),
            Room::ThreeExit => self.render_room_three_exit(),
            Room::TwoExitWithMazeExit => self.render_room_two_exit_with_maze_exit(),
            Room::TwoExit => self.render_room_two_exit(),
        }
    }

    /// Stack shapes into a stack where each vector is divided by a new layer
    pub fn stack(&self, layers: Vec<Vec<Shape>>) -> Vec<Shape> {
        let mut stack = Vec::new();
        for layer in layers.into_iter() {
            stack.extend(layer);
            stack.push(Shape::Layer);
        }
        stack
    }

    fn render_room_corridor(&self) -> Vec<Shape> {
        let half_width = self.width / 2.0;
        let left_wall_x = half_width - (half_width * 0.30);

        // lines from left to right
        let mut shapes = Vec::new();
        // vertical lines
        let start_y = self.x_scale * 6.0;
        for i in 0..6 {
            let incr = (i as f64) * start_y;
            shapes.push(Shape::Line(Line {
                x1: 0.0,
                x2: left_wall_x,
                y1: incr,
                y2: left_wall_x + incr,
                color: Color::DarkGray,
            }))
        }
        // horizontal lines
        let start_x = self.y_scale * 4.0;
        for i in 0..4 {
            let incr = (i as f64) * start_x;
            shapes.push(Shape::Line(Line {
                x1: incr,
                x2: left_wall_x,
                y1: 0.0,
                y2: left_wall_x - incr,
                color: Color::DarkGray,
            }))
        }

        // lines from right to left
        let right_wall_x2 = self.width;
        let right_wall_x = right_wall_x2 - half_width + (half_width * 0.30);
        // vertical lines
        let start_y = self.x_scale * 6.0;

        for i in 0..6 {
            let incr = (i as f64) * start_y;
            shapes.push(Shape::Line(Line {
                x1: right_wall_x,
                x2: right_wall_x2,
                y1: left_wall_x + incr,
                y2: incr,
                color: Color::DarkGray,
            }))
        }
        // horizontal lines
        let start_x = self.y_scale * 4.0;
        for i in 0..4 {
            let incr = (i as f64) * start_x;
            let x1 = right_wall_x2 - incr;
            shapes.push(Shape::Line(Line {
                x1,
                x2: right_wall_x,
                y1: 0.0,
                y2: left_wall_x - incr,
                color: Color::DarkGray,
            }))
        }

        shapes
    }

    fn render_room_corridor_with_maze_exit(&self) -> Vec<Shape> {
        let mut shapes = self.render_room_corridor();
        shapes.push(Shape::Layer);
        let half_width = self.width / 2.0;
        let door_width = half_width - (half_width * 0.80);
        shapes.push(Shape::Rectangle(Rectangle {
            x: 2.0 * self.x_scale,
            y: 0.0,
            width: door_width,
            height: 15.0 * self.y_scale,
            color: Color::Red,
        }));
        shapes
    }

    fn render_room_two_exit(&self) -> Vec<Shape> {
        let mut shapes = self.render_room_three_exit();
        let half_width = self.width / 2.0;

        // @! wall
        let front_wall_x = half_width - (half_width * 0.30);
        let front_wall_x2 = self.width - half_width + (half_width * 0.30);
        for i in 0..(self.height as i32) {
            let y = i as f64 * self.y_scale;
            shapes.push(Shape::Line(Line {
                x1: front_wall_x,
                x2: front_wall_x2,
                y1: y,
                y2: y,
                color: Color::DarkGray,
            }))
        }

        shapes
    }

    fn render_room_two_exit_with_maze_exit(&self) -> Vec<Shape> {
        let mut shapes = self.render_room_two_exit();

        shapes.push(Shape::Layer);
        let half_width = self.width / 2.0;
        let door_width = half_width - (half_width * 0.40);
        let door_x = half_width - (door_width / 2.0);
        let door_y = (self.height / 2.0) + (15.0 * self.y_scale);
        shapes.push(Shape::Rectangle(Rectangle {
            x: door_x,
            y: door_y - 40.0,
            width: door_width,
            height: 80.0,
            color: Color::Green,
        }));

        shapes
    }

    fn render_room_three_exit(&self) -> Vec<Shape> {
        let mut shapes = Vec::new();
        let half_width = self.width / 2.0;
        // lines from left to right
        let left_wall_x = half_width - (half_width * 0.80);

        // vertical lines
        let start_y = self.x_scale * 6.0;
        for i in 0..6 {
            let incr = (i as f64) * start_y;
            shapes.push(Shape::Line(Line {
                x1: 0.0,
                x2: left_wall_x,
                y1: incr,
                y2: left_wall_x + incr,
                color: Color::DarkGray,
            }))
        }
        // horizontal lines
        let start_x = self.y_scale * 4.0;
        for i in 0..4 {
            let incr = (i as f64) * start_x;
            shapes.push(Shape::Line(Line {
                x1: incr,
                x2: left_wall_x,
                y1: 0.0,
                y2: left_wall_x - incr,
                color: Color::DarkGray,
            }))
        }

        // @! door

        // redo
        // lines from left to right
        let left_wall_x1 = half_width - (half_width * 0.50);
        let left_wall_x2 = half_width - (half_width * 0.30);

        // vertical lines
        let start_y = self.x_scale * 6.0;
        for i in 0..6 {
            let incr = (i as f64) * start_y;
            shapes.push(Shape::Line(Line {
                x1: left_wall_x1,
                x2: left_wall_x2,
                y1: incr + left_wall_x1,
                y2: left_wall_x2 + incr,
                color: Color::DarkGray,
            }))
        }
        // horizontal lines
        let start_x = self.y_scale * 4.0;
        for i in 0..4 {
            let incr = (i as f64) * start_x;
            shapes.push(Shape::Line(Line {
                x1: left_wall_x1 + incr,
                x2: left_wall_x2,
                y1: 0.0,
                y2: left_wall_x - incr,
                color: Color::DarkGray,
            }))
        }

        // lines from right to left
        let right_wall_x2 = self.width;
        let right_wall_x = right_wall_x2 - half_width + (half_width * 0.80);
        // vertical lines
        let start_y = self.x_scale * 6.0;

        for i in 0..6 {
            let incr = (i as f64) * start_y;
            shapes.push(Shape::Line(Line {
                x1: right_wall_x,
                x2: right_wall_x2,
                y1: left_wall_x + incr,
                y2: incr,
                color: Color::DarkGray,
            }))
        }
        // horizontal lines
        let start_x = self.y_scale * 4.0;
        for i in 0..4 {
            let incr = (i as f64) * start_x;
            let x1 = right_wall_x2 - incr;
            shapes.push(Shape::Line(Line {
                x1,
                x2: right_wall_x,
                y1: 0.0,
                y2: left_wall_x - incr,
                color: Color::DarkGray,
            }))
        }

        // @! door

        // redo
        // lines from left to right
        let right_wall_x1 = self.width - half_width + (half_width * 0.50);
        let right_wall_x2 = self.width - half_width + (half_width * 0.30);

        // vertical lines
        let start_y = self.x_scale * 6.0;
        for i in 0..6 {
            let incr = (i as f64) * start_y;
            shapes.push(Shape::Line(Line {
                x1: right_wall_x1,
                x2: right_wall_x2,
                y1: left_wall_x1 + incr,
                y2: incr + left_wall_x2,
                color: Color::DarkGray,
            }))
        }
        // horizontal lines
        let start_x = self.y_scale * 4.0;
        for i in 1..2 {
            let incr = (i as f64) * start_x;
            let x1 = right_wall_x2 + incr;
            shapes.push(Shape::Line(Line {
                x1,
                x2: right_wall_x2,
                y1: 0.0,
                y2: left_wall_x1 - incr,
                color: Color::DarkGray,
            }))
        }

        shapes
    }
}
