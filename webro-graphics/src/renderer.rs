#[allow(unused_imports)]
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};
#[allow(unused_imports)]
use crate::prelude::{Vertex, Canvas, VertexBuffer, IndexBuffer, USAGE ,GameState, ShaderDataType, BufferLayout, BufferData, VertexArray};
use webro_assets::prelude::{Shader, SPRITE, Texture};
use webro_math::prelude::{Vector3, Vector2};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use webro_logging::prelude::log;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub static mut RENDERER : Option<Renderer> = None;

#[allow(dead_code)]
pub struct Renderer {
    vertex_buffers : Vec<VertexBuffer>, 
    index_buffers : Vec<IndexBuffer>, 
    shaders : Vec<Shader>, 
    textures : Vec<Texture>,
    canvas : Canvas,
    max_indices : u32, 
    max_quads : u32, 
    max_vertices : u32, 
    max_textures : u32,
    current : usize,
    data : BufferData, 
    vao : Option<VertexArray>
}

const MAXQUADS : u32 = 425;
const MAXTEXTURE : u32 = 16;

impl Default for Renderer { 
    fn default() -> Self {

        Self {
            vertex_buffers : Vec::new(),
            index_buffers : Vec::new(), 
            shaders : Vec::new(),
            textures : Vec::new(),
            canvas : Canvas::new("canvas"),
            max_quads : MAXQUADS,
            max_vertices : MAXQUADS * 4,
            max_indices : MAXQUADS * 6,
            max_textures : MAXTEXTURE,
            current : 0, 
            data : BufferData::new(), 
            vao : None
        }
    }
}

impl Renderer {

    pub fn new(canvas_id : &str) -> Self {

        Self {
            vertex_buffers : Vec::new(),
            index_buffers : Vec::new(), 
            shaders : Vec::new(),
            textures : Vec::new(),
            canvas : Canvas::new(canvas_id),
            max_quads : MAXQUADS,
            max_vertices : MAXQUADS * 4,
            max_indices : MAXQUADS * 6,
            max_textures : MAXTEXTURE,
            current : 0, 
            data : BufferData::new(), 
            vao : None
        }
    }

    pub fn bind(&mut self){

        if self.vao.is_none() {self.vao = Some(VertexArray::new(&self.canvas.gl))}
        else {self.vao.as_ref().unwrap().bind(&self.canvas.gl)}

        self.textures[self.current].bind(&self.canvas.gl);
        self.shaders[self.current].bind(&self.canvas.gl);
        self.vertex_buffers[self.current].bind(&self.canvas.gl, self.shaders[self.current].get_program(), USAGE::STATIC);
        self.index_buffers[self.current].bind(&self.canvas.gl);
        
        self.draw(self.current);
        
        self.vao.as_ref().unwrap().unbind(&self.canvas.gl);
        self.textures[self.current].unbind(&self.canvas.gl);
        self.shaders[self.current].unbind(&self.canvas.gl);
        self.vertex_buffers[self.current].unbind(&self.canvas.gl);
        self.index_buffers[self.current].unbind(&self.canvas.gl);


    }

    pub fn test(&mut self) {
        self.begin_scene();
        self.draw_quad(Vector3::default(), Vector3::new( 0.0, 1.0, 1.0), 0.0);
        self.draw_quad(Vector3::new(-1.0, -1.0, 0.0), Vector3::new( 1.0, 1.0, 1.0), 1.0);
        self.draw_quad(Vector3::new(1.0, 1.0, 0.0), Vector3::new( 0.0, 1.0, 1.0), 2.0);
        self.end_scene();

    }

    pub fn render(&'static mut self ) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let s = Rc::new(RefCell::new(self));
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            s.borrow_mut().test();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    pub fn begin_scene(&mut self){
        self.clear();
        self.start_batch();

        if self.textures.is_empty() {
            let texture = Texture::new();
            self.textures.push(texture);
            self.textures[self.current].init(&self.canvas.gl);
            self.textures[self.current].add_texture(&self.canvas.gl, "./cp437.png");
            self.textures[self.current].add_texture(&self.canvas.gl, "https://webglfundamentals.org/webgl/resources/f-texture.png");
        }
        if self.shaders.is_empty(){
            let shader = Shader::new(SPRITE::V_SHADER, SPRITE::F_SHADER);
            self.shaders.push(shader);
            self.shaders[self.current].compile(&self.canvas.gl);
            self.shaders[self.current].set_int(&self.canvas.gl, "u_textures", &self.textures[self.current].get_slot_array());
        }
    }

    pub fn end_scene(&mut self){
        self.flush();
        self.vertex_buffers.clear();
        self.index_buffers.clear();
        self.current = 0;
    }

    pub fn flush(&mut self){
        let vertices = self.vertex_initialize();
        let mut v_buffer = VertexBuffer::new(&vertices);
        v_buffer.add_layout(BufferLayout::new(ShaderDataType::FLOAT3, "position"));
        v_buffer.add_layout(BufferLayout::new(ShaderDataType::FLOAT3, "color"));
        v_buffer.add_layout(BufferLayout::new(ShaderDataType::FLOAT2, "texcoord"));
        v_buffer.add_layout(BufferLayout::new(ShaderDataType::FLOAT, "tex_id"));
        self.vertex_buffers.push(v_buffer);
        drop(vertices);
        
        let indices = self.indice_initialize();
        let i_buffer = IndexBuffer::new(&indices);
        self.index_buffers.push(i_buffer);
        drop(indices);
        self.bind();
    }

    fn clear(&self){
        self.canvas.gl.clear_color(1.0, 0.0, 0.0, 1.0);
        self.canvas.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    fn draw(&self, id : usize){
        self.canvas.gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES,
            self.index_buffers[id].size(),
            WebGl2RenderingContext::UNSIGNED_SHORT,
        0);
    }
    
    pub fn draw_quad(&mut self, position : Vector3<f32>, color : Vector3<f32>, slot : f32){
        if self.data.id >= self.max_quads {
            self.next_batch();
        };

        let id = self.data.id;
        self.data.id = id + 1;


        let positions : [Vector3<f32>; 4] = [
            Vector3::new(position.x - 0.5, position.y + 0.5, position.z),  // top left
            Vector3::new(position.x + 0.5, position.y + 0.5, position.z),  // top right
            Vector3::new(position.x + 0.5, position.y - 0.5, position.z),  // bottom right
            Vector3::new(position.x - 0.5, position.y - 0.5, position.z),  // bottom left
        ];
        
        let texcoords : [Vector2<f32>; 4] = [
            Vector2::new(0.0, 1.0), // top left
            Vector2::new(1.0, 1.0), // top right
            Vector2::new(1.0, 0.0), // bottom right
            Vector2::new(0.0, 0.0), // bottom left
        ];

        let mut i = 0;
        self.data.quad_buffer_ptr = 4 * id as usize;
        for (position , texcoord) in positions.iter().zip(texcoords.iter()) {
            self.data.quad_buffer.push(Vertex::default());
            self.data.quad_buffer[self.data.quad_buffer_ptr + i].position = *position;
            self.data.quad_buffer[self.data.quad_buffer_ptr + i].color = color;
            self.data.quad_buffer[self.data.quad_buffer_ptr + i].texcoord = *texcoord;
            self.data.quad_buffer[self.data.quad_buffer_ptr + i].tex_id = slot;
            i+=1;
        }

    }

    pub fn vertex_initialize(&mut self) -> [f32; 425 * 4 * 3 * 3* 3]{
        let count = 9;
        let mut vertices = [0.0; 425 * 4 * 3 * 3* 3];
        for i in 0..self.data.quad_buffer.len(){
            // position
            vertices[0 + (count * i) as usize] = self.data.quad_buffer[i as usize].position.x;
            vertices[1 + (count * i) as usize] = self.data.quad_buffer[i as usize].position.y;
            vertices[2 + (count * i) as usize] = self.data.quad_buffer[i as usize].position.z;
            // color
            vertices[3 + (count * i) as usize] = self.data.quad_buffer[i as usize].color.x;
            vertices[4 + (count * i) as usize] = self.data.quad_buffer[i as usize].color.y;
            vertices[5 + (count * i) as usize] = self.data.quad_buffer[i as usize].color.z;
            // texcoord
            vertices[6 + (count * i) as usize] = self.data.quad_buffer[i as usize].texcoord.x;
            vertices[7 + (count * i) as usize] = self.data.quad_buffer[i as usize].texcoord.y;
            // tex_id
            vertices[8 + (count * i) as usize] = self.data.quad_buffer[i as usize].tex_id;

        };
        vertices
    }
    
    pub fn indice_initialize(&mut self) -> [u16; 425 * 6]{
        let mut offset = 0;
        let mut indices = [0; 425 * 6];
        for i in (0..self.max_indices).step_by(6){
            indices[0 + i as usize] = 0 + offset; // 0
            indices[1 + i as usize] = 1 + offset; // 1
            indices[2 + i as usize] = 2 + offset; // 2
            indices[3 + i as usize] = 3 + offset; // 3
            indices[4 + i as usize] = 2 + offset; // 2 
            indices[5 + i as usize] = 0 + offset; // 0
            offset += 4;
        };
        indices
    }

    fn start_batch(&mut self){
        self.data.clear();
    }
    
    fn next_batch(&mut self){
        self.flush();
        self.current += 1;
        self.start_batch();
    }
}
