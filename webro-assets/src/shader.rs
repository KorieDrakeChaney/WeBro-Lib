#![allow(dead_code, non_camel_case_types)]
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use webro_math::prelude::{Mat4, Vector4, Vector3, Vector2};
use webro_logging::prelude::log;

enum Type{
    VERTEX_SHADER,
    FRAGMENT_SHADER
}

pub struct Shader { 
    v_source : &'static str, 
    f_source : &'static str, 
    program : Option<WebGlProgram>
} 

impl Shader { 

    pub fn new(v_source : &'static str, f_source : &'static str) -> Self {
        Self {
            v_source, 
            f_source, 
            program : None, 
        }   
    }

    pub fn compile(&mut self, gl : &WebGl2RenderingContext){
        let v_shader : Option<WebGlShader> = compile_shader(gl, self.v_source, Type::VERTEX_SHADER);
        let f_shader : Option<WebGlShader> = compile_shader(gl, self.f_source, Type::FRAGMENT_SHADER);

        if v_shader.is_none(){
            log("Error creating v_shader");
            return;
        }

        if f_shader.is_none(){
            log("Error creating f_shader");
            return;
        }
        
        self.link_program(gl, v_shader.as_ref().unwrap(), f_shader.as_ref().unwrap());

        delete_shader(gl, v_shader.unwrap());
        delete_shader(gl, f_shader.unwrap());
    }

    pub fn set_mat4(&self, gl : &WebGl2RenderingContext, name : &str , value : &Mat4<f32>){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, &[value.x.x, value.x.y, value.x.z, value.x.w, 
                                                                   value.y.x, value.y.y, value.y.z, value.y.w,
                                                                   value.z.x, value.z.y, value.z.z, value.z.w, 
                                                                   value.w.x, value.w.y, value.w.z, value.w.w,
                                                                   ]);
    }

    pub fn set_float4(&self, gl : &WebGl2RenderingContext, name : &str, value : &[f32]){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
        if loc.is_none() {
            log("Error");
        };
        gl.uniform4fv_with_f32_array(loc.as_ref(), value);
                                                                  
    }

    pub fn set_float3(&self, gl : &WebGl2RenderingContext, name : &str, value : &[f32]){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform3fv_with_f32_array(loc.as_ref(), value);
    }

    pub fn set_float2(&self, gl : &WebGl2RenderingContext, name : &str, value : &[f32]){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform2fv_with_f32_array(loc.as_ref(), value);
    }

    pub fn set_float(&self, gl : &WebGl2RenderingContext, name : &str, value : &[f32]){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform1fv_with_f32_array(loc.as_ref(), value);
    }

    pub fn set_int4(&self, gl : &WebGl2RenderingContext, name : &str, value : &[i32]){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform4iv_with_i32_array(loc.as_ref(), value);
                                                                  
    }

    pub fn set_int3(&self, gl : &WebGl2RenderingContext, name : &str, value : &[i32]){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform3iv_with_i32_array(loc.as_ref(), value);
    }

    pub fn set_int2(&self, gl : &WebGl2RenderingContext, name : &str, value : &[i32]){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform2iv_with_i32_array(loc.as_ref(), value);
    }

    pub fn set_int(&self, gl : &WebGl2RenderingContext, name : &str, value : &[i32]){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform1iv_with_i32_array(loc.as_ref(), value);
    }

    pub fn set_int_js(&self, gl : &WebGl2RenderingContext, name : &str, value : &wasm_bindgen::JsValue){
        gl.use_program(Some(self.program.as_ref().unwrap()));

        let loc = gl.get_uniform_location(self.program.as_ref().unwrap(), name);
    
        if loc.is_none() {
            // error("location failed ! ");
        };
        gl.uniform1iv_with_i32_sequence(loc.as_ref(), value);
    }


    pub fn get_program(&self) -> &WebGlProgram { 
        self.program.as_ref().unwrap()
    }


    pub fn link_program(
        &mut self, gl : &WebGl2RenderingContext, v_shader : &WebGlShader, f_shader : &WebGlShader
    ) {
        let program = gl
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object")).unwrap();
    
        gl.attach_shader(&program, v_shader);
        gl.attach_shader(&program, f_shader);
        gl.link_program(&program);
    
        if gl
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            self.program = Some(program);
        } 
    }
    
    pub fn bind(&self, gl : &WebGl2RenderingContext){
        gl.use_program(Some(self.program.as_ref().unwrap()));
    }

    pub fn unbind(&self, gl : &WebGl2RenderingContext){
        gl.use_program(None);
    }


}

fn delete_shader(gl : &WebGl2RenderingContext, shader : WebGlShader){
    gl.delete_shader(Some(shader.as_ref()));
}

fn compile_shader(gl : &WebGl2RenderingContext, source : &'static str, shader_type : Type) -> Option<WebGlShader>{
    let shader = match shader_type {
        Type::VERTEX_SHADER => WebGl2RenderingContext::VERTEX_SHADER,
        Type::FRAGMENT_SHADER => WebGl2RenderingContext::FRAGMENT_SHADER,
    };

    let compiled_shader = gl
    .create_shader(shader)
    .ok_or_else(|| String::from("Unable to create shader object")).unwrap();
    gl.shader_source(&compiled_shader, source);
    gl.compile_shader(&compiled_shader);

    if gl
        .get_shader_parameter(&compiled_shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Some(compiled_shader)
    } 
    else {
        None
    }
}
