
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShaderDataType { 
    FLOAT, FLOAT2, FLOAT3, FLOAT4, 
    MAT2, MAT3, MAT4, 
    VEC2, VEC3, VEC4, 
    BOOL
}

pub fn match_data_type(data_type : ShaderDataType) -> i32{
    match data_type { 
        ShaderDataType::BOOL => 2, 
        ShaderDataType::FLOAT => 4 * 1, 
        ShaderDataType::FLOAT2 => 4 * 2, 
        ShaderDataType::FLOAT3 => 4 * 3, 
        ShaderDataType::FLOAT4 => 4 * 4, 
        ShaderDataType::MAT2 => 4 * 2 * 2, 
        ShaderDataType::MAT3 => 4 * 3 * 3, 
        ShaderDataType::MAT4 => 4 * 4 * 4, 
        ShaderDataType::VEC2 => 4 * 2, 
        ShaderDataType::VEC3 => 4 * 3, 
        ShaderDataType::VEC4 => 4 * 4,
    }
}
