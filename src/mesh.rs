/*
pub enum GeometryType{
    Triangles
}

impl GeometryType{
    pub fn vertices(&self) -> usize{
        match *self{
            GeometryType::Triangles => 3,
        }
    }
}

pub struct VertexP3{
    pub p:[f32; 3],
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Clone)]
pub struct VertexP3N3{
    pub p:[f32; 3],
    pub n:[f32; 3],
}

pub struct VertexP3N3T0C2{
    pub p:[f32; 3],
    pub n:[f32; 3],
    pub tc:[f32;2],
}

pub struct VertexP3N3Bone{
    pub p:[f32; 3],
    pub n:[f32; 3],
    pub bone:usize,
}

pub struct VertexP3N3T0C2Bone{
    pub p:[f32; 3],
    pub n:[f32; 3],
    pub tc:[f32;2],
    pub bone:usize,
}

pub struct Lod<V>{
    pub distance:f32,
    pub vertices:Vec<V>,
}
*/

pub struct Mesh{
    //material:String,
    name:String,
    //geometryType
    //vertexFormat:String,
    //lods:Lod<V>,
}
