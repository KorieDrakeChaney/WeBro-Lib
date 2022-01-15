use std::collections::HashMap;

pub struct AssetManager { 
    texture_ids : HashMap<String, i32>,
    shader_ids : HashMap<String, i32>
}

impl AssetManager { 
    pub fn new() -> Self{
        Self { 
            texture_ids : HashMap::with_capacity(16), 
            shader_ids : HashMap::new(),
        }
    }

    pub fn insert_texture(&mut self, name : &str, id : i32){
        self.texture_ids.insert(name.to_string(), id);
    }

    pub fn insert_shader(&mut self, name : &str, id : i32){
        self.shader_ids.insert(name.to_string(), id);
    }

    pub fn has_texture_name(&self, name : &str) -> bool{
        self.texture_ids.contains_key(name)
        
    }

    pub fn has_shader_name(&self, name : &str) -> bool{
        self.shader_ids.contains_key(name)
    }

    pub fn has_texture_id(&self, id : i32) -> bool{
        self.texture_ids.values().any(|&val| val == id)
    }

    pub fn has_shader_id(&self, id : i32) -> bool{
        self.shader_ids.values().any(|&val| val == id)
    }

    pub fn clear(&mut self){
        self.texture_ids.drain();
        self.shader_ids.drain();
    }
}