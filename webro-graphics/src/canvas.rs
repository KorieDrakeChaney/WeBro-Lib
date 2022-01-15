use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext};

#[derive(Debug, Clone)]
pub struct Canvas { 
    pub gl : web_sys::WebGl2RenderingContext
}

impl Canvas { 
    pub fn new(canvas_id : &str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>().unwrap();
            
            gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        Self {
            gl 
        }

    }

}

