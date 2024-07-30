mod sketch;
mod canvas;
mod color;
mod shape;
mod point;
mod pixelbuffer;

use sketch::Sketch;

fn main() -> Result<(), String> {
    let mut sketch = Sketch::new()?;
    sketch.run();
    Ok(())
}