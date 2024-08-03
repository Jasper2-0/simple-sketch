mod sketch;
mod canvas;
mod color;
mod shape;
mod geom;
mod pixelbuffer;

use sketch::Sketch;

fn main() -> Result<(), String> {
    let mut sketch = Sketch::new()?;
    sketch.run();
    Ok(())
}