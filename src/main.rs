mod application;
mod sketch;
mod canvas;
mod color;
mod shape;

use sketch::Sketch;
use application::ApplicationRunner;

fn main() -> Result<(), String> {
    let mut runner = ApplicationRunner::<Sketch>::new()?;
    runner.run();
    Ok(())
}