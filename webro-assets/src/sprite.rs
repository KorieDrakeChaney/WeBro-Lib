#[allow(non_snake_case)]


pub mod SPRITE {
    pub const V_SHADER: &str =
    "   #version 300 es
     
        layout(location = 0) in vec3 position;
        layout(location = 1) in vec3 color;
        layout(location = 2) in vec2 texcoord;
        layout(location = 3) in float tex_id;
        
        out vec3 v_color;
        out vec2 v_texcoord;
        flat out float v_tex_id;

        void main() {
            v_color = color;
            v_texcoord = texcoord;
            v_tex_id = tex_id;
            gl_Position = vec4(position, 1);
        }
    ";
    
    pub const F_SHADER: &str =
    "   #version 300 es
        precision highp float;

        in vec3 v_color;
        in vec2 v_texcoord;
        flat in float v_tex_id;

        out vec4 outColor;

        uniform sampler2D u_textures[16];

        vec4 getSampleFromArray(sampler2D textures[16], int index, vec2 uv) {
            vec4 c = vec4(0., 0., 0., 0.);
            if (index == 0){
                c = texture(u_textures[0], uv);
            }
            if (index == 1){
                c = texture(u_textures[1], uv);
            }
            if (index == 2){
                c = texture(u_textures[2], uv);
            }
            if (index == 3){
                c = texture(u_textures[3], uv);
            }
            if (index == 4){
                c = texture(u_textures[4], uv);
            }
            if (index == 5){
                c = texture(u_textures[5], uv);
            }
            if (index == 6){
                c = texture(u_textures[6], uv);
            }         
            if (index == 7){
                c = texture(u_textures[7], uv);
            }
            if (index == 8){
                c = texture(u_textures[8], uv);
            }
            if (index == 9){
                c = texture(u_textures[9], uv);
            }         
            if (index == 10){
                c = texture(u_textures[10], uv);
            }
            if (index == 11){
                c = texture(u_textures[11], uv);
            }
            if (index == 12){
                c = texture(u_textures[12], uv);
            }
            if (index == 13){
                c = texture(u_textures[13], uv);
            } 
            if (index == 14){
                c = texture(u_textures[14], uv);
            }
            if (index == 15){
                c = texture(u_textures[15], uv);
            }
            return c;
        }

        void main() {
            outColor = getSampleFromArray(u_textures, int(v_tex_id), v_texcoord) * vec4(v_color, 1.0);
        }
    ";
    
    pub const ALL : (&str, &str) = (V_SHADER, F_SHADER);
}

