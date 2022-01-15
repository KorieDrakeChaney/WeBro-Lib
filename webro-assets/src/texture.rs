use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlImageElement, WebGl2RenderingContext, WebGlTexture,
};


#[derive(Clone)]
pub struct Texture { 
    textures : [Option<WebGlTexture> ; 32], 
    pub texture_count : u32,
    pub current_texture : usize
}


impl Texture {
    pub fn new() -> Self { 
        Self { 
            textures : Default::default(), 
            texture_count : 0,
            current_texture : 0
        }
    }

    pub fn init(&mut self, gl : &WebGl2RenderingContext){

        let texture = gl.create_texture().expect("Cannot create gl texture");
        self.textures[0] = Some(texture.clone());
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        let pixel: [u8; 4] = [255, 255, 255, 255];
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            1,
            1,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&pixel),
        ).unwrap();
        
        
    }

    pub fn get_slot_array(&self) -> [i32 ; 32] {
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 
         11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 
         21, 22, 23, 24, 25, 26, 27, 28, 29, 
         30, 31]
    }

    pub fn add_texture(&mut self, gl : &WebGl2RenderingContext, link : &str){
        self.texture_count += 1;
        self.current_texture += 1;
        self.textures[self.current_texture] = Some(self.load_texture(gl, link, self.current_texture).unwrap());
    }

    pub fn set_texture_at(&mut self, gl : &WebGl2RenderingContext, link : &str, slot : usize){
        self.textures[slot] = Some(self.load_texture(gl, link, slot).unwrap());
    }

    pub fn load_texture(&mut self, gl : &WebGl2RenderingContext, link : &str, slot : usize) -> Result<WebGlTexture, JsValue>{
        let texture = gl.create_texture().expect("Cannot create gl texture");
        let t = Some(texture.clone());
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        let pixel: [u8; 4] = [0, 0, 0, 255];
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            1,
            1,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&pixel),
        )?;


        let img = HtmlImageElement::new().unwrap();
        img.set_cross_origin(Some(""));
        
        let imgrc = Rc::new(img);
        
        let texture = Rc::new(texture);

        {
            let img = imgrc.clone();
            let texture = Rc::new(texture);
            let gl = Rc::new(gl.clone());
            let a = Closure::wrap(Box::new(move || {
                gl.active_texture(WebGl2RenderingContext::TEXTURE0 + slot as u32);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
                gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    WebGl2RenderingContext::RGBA as i32,
                    WebGl2RenderingContext::RGBA,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    &img,
                ).unwrap();
                gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
            }) as Box<dyn FnMut()>);
            
            imgrc.set_onload(Some(a.as_ref().unchecked_ref()));

            a.forget();
        }
        
        imgrc.set_src(link);

        Ok(t.unwrap())
    }
    pub fn bind(&mut self, gl : &WebGl2RenderingContext){
        for (i, texture) in self.textures.iter().enumerate() {
            if texture.is_some() {
                gl.active_texture(WebGl2RenderingContext::TEXTURE0 + i as u32);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture.as_ref().unwrap()));
            }
        }
    }

    pub fn unbind(&self, gl : &WebGl2RenderingContext){
        for (i, texture) in self.textures.iter().enumerate() {
            if texture.is_some() {
                gl.active_texture(WebGl2RenderingContext::TEXTURE0 + i as u32);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
            }
        }

    }
}
