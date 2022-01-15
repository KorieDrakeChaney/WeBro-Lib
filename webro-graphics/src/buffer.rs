use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlBuffer, WebGlVertexArrayObject};
use crate::prelude::{ShaderDataType, match_data_type};
use webro_math::prelude::{Vector3, Vector2};

#[allow(non_camel_case_types)]
pub enum USAGE{
    STATIC, 
    DYNAMIC, 
    STATIC_READ, 
    DYNAMIC_READ, 
    STATIC_COPY, 
    DYNAMIC_COPY
}


fn usage_convert(usage : USAGE) -> u32{
    match usage { 
        USAGE::STATIC => WebGl2RenderingContext::STATIC_DRAW, 
        USAGE::DYNAMIC => WebGl2RenderingContext::DYNAMIC_DRAW, 
        USAGE::STATIC_READ => WebGl2RenderingContext::STATIC_READ, 
        USAGE::DYNAMIC_READ => WebGl2RenderingContext::DYNAMIC_READ, 
        USAGE::STATIC_COPY => WebGl2RenderingContext::STATIC_COPY, 
        USAGE::DYNAMIC_COPY => WebGl2RenderingContext::DYNAMIC_COPY, 
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BufferLayout {
    data_type : ShaderDataType, 
    name : &'static str
}

impl BufferLayout {

    pub fn new(data_type : ShaderDataType, name : &'static str) -> Self { 
        Self {
            data_type, 
            name
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position : Vector3<f32>,
    pub color : Vector3<f32>,
    pub texcoord : Vector2<f32>, 
    pub tex_id : f32
}

impl Default for Vertex { 
    fn default() -> Self {
        Self { 
            position : Vector3::new(0.0, 0.0, 0.0),
            color : Vector3::new(0.0, 0.0, 0.0),
            texcoord : Vector2::new(0.0, 0.0),
            tex_id : 0.0
        }
    }
}

pub struct BufferData {
    pub quad_buffer : Vec<Vertex>,
    pub quad_buffer_ptr : usize,
    pub index_count : u32,
    pub id : u32,
    pub buffer_id : u32
}

impl BufferData {
    pub fn new() -> Self {
        Self {
            quad_buffer : vec![],
            quad_buffer_ptr : 0, 
            index_count : 0, 
            id : 0,
            buffer_id : 0
        }
    }

    pub fn clear(&mut self){
        self.quad_buffer.clear(); 
        self.quad_buffer_ptr = 0;
        self.index_count = 0;
        self.id = 0;
        self.buffer_id = 0;
    }
}

pub struct VertexArray {
    pub vao : WebGlVertexArrayObject
}

impl VertexArray {
    pub fn new(gl : &WebGl2RenderingContext) -> Self {
        let vao = gl
        .create_vertex_array()
        .ok_or("Could not create vertex array object").unwrap();
        gl.bind_vertex_array(Some(&vao));
        Self { 
            vao
        }
    }

    pub fn bind(&self, gl : &WebGl2RenderingContext){
        gl.bind_vertex_array(Some(&self.vao));
    }

    pub fn delete(&self, gl : &WebGl2RenderingContext){
        gl.delete_vertex_array(Some(&self.vao));
    }

    pub fn unbind(&self, gl : &WebGl2RenderingContext){
        gl.bind_vertex_array(None);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VertexBuffer {
    buffer_layout : Vec<BufferLayout>, 
    pub buffer : Option<WebGlBuffer>,
    data : [f32 ; 425 * 4 * 3 * 3 * 3],
    size : i32
}

impl VertexBuffer {
    pub fn new(data : &[f32 ; 425 * 4 * 3 * 3 * 3]) -> Self {
        Self { 
            buffer_layout : vec![],
            buffer : None,
            data : *data,
            size : 0
        }
    }

    pub fn add_layout(&mut self, layout : BufferLayout){
        self.buffer_layout.push(layout);
        self.size += match_data_type(layout.data_type) / 4;
    }

    pub fn size(&self) -> i32{
        self.data.len() as i32
    }

    pub fn set_data(&mut self, data : &[f32 ; 425 * 4 * 3 * 3 * 3]){
        self.data = *data;
    }

    pub fn bind(&mut self, gl : &WebGl2RenderingContext, program : &WebGlProgram, usage : USAGE){
        if self.buffer.is_none(){
            self.buffer = Some(gl.create_buffer().ok_or("Failed to create buffer").unwrap());
        }
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));

        unsafe {
            let attrib_buf = js_sys::Float32Array::view(&self.data);
            
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &attrib_buf,
                usage_convert(usage),
            );
        }
        
        let mut offset = 0;
        
        for attrib in &self.buffer_layout {
            let value = match_data_type(attrib.data_type);
            let size = value / 4;
            let attrib_location : i32 = gl.get_attrib_location(program, attrib.name);
        
            gl.vertex_attrib_pointer_with_i32(attrib_location as u32, size, WebGl2RenderingContext::FLOAT, false, self.size * 4, offset * 4);
            gl.enable_vertex_attrib_array(attrib_location as u32);
            offset += size;
        }

    }

    pub fn delete(&mut self, gl : &WebGl2RenderingContext){
        gl.delete_buffer(self.buffer.as_ref());
    }

    pub fn unbind(&self, gl : &WebGl2RenderingContext){
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexBuffer {
    data : [u16 ; 425 * 6], 
    buffer : Option<WebGlBuffer>
}

impl IndexBuffer {
    pub fn new(data : &[u16 ; 425 * 6]) -> Self {
        Self { 
            data: *data, 
            buffer : None
        }
    }

    pub fn set_data(&mut self, data : &[u16 ; 425 * 6]){
        self.data = *data;
    }

    pub fn size(&self) -> i32{
        self.data.len() as i32
    }

    pub fn bind(&mut self, gl : &WebGl2RenderingContext ){
        if self.buffer.is_none() {
            self.buffer = Some(gl.create_buffer().ok_or("Failed to create buffer").unwrap());
        }
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.buffer.as_ref().unwrap()));

        unsafe {
            let attrib_buf = js_sys::Uint16Array::view(&self.data);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &attrib_buf,
                WebGl2RenderingContext::STATIC_DRAW,
                );
        }
    }
    
    pub fn delete(&mut self, gl : &WebGl2RenderingContext){
        gl.delete_buffer(self.buffer.as_ref());
    }

    pub fn unbind(&self, gl : &WebGl2RenderingContext){
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }
}


