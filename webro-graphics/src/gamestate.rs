use web_sys::{WebGl2RenderingContext};

pub trait GameState: 'static {
    fn tick(&mut self, gl: &WebGl2RenderingContext);
}