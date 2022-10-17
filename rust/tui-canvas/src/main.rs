mod canvas;
mod model;
mod render;

use model::Model;
use render::Render;

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

    let (x, y) = model.origin()?;
    let y = Render::origin_y(y);
    let shape = Render::render(x, y, &shape_str)?;

    model.mount_canvas(&shape)?;
    model.run()?;

    Ok(())
}
