mod canvas;
mod model;
mod render;

use model::Model;
use render::{Render, Room};
use tuirealm::props::Color;

use std::fs::File;
use std::io::Read;

#[derive(Eq, PartialEq)]
pub enum Msg {
    Quit,
    None,
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Canvas,
}

fn read_shape_file(file: &str) -> anyhow::Result<String> {
    let mut file = File::open(file)?;
    let mut shape = String::new();
    file.read_to_string(&mut shape)?;
    Ok(shape)
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("usage: tui-canvas <shape_file>");
    }
    let shape_str = read_shape_file(&args[1])?;
    let mut model = Model::default();
    let (width, height) = model.size()?;
    let render = Render::new(width, height);

    let item_x = (width / 2.0) - ((shape_str.len() / 2) as f64);
    let shape = render.ascii_art(item_x, height, &shape_str, Color::Yellow);
    let room = render.render_room(Room::CorridorWithMazeExit);
    let shapes = render.stack(vec![room, shape]);

    model.mount_canvas(&shapes)?;
    model.run()?;

    Ok(())
}
