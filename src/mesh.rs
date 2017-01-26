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

use LOD;
use collada;

pub enum GeometryType{
    Lines,
    Triangles,
}

pub struct Mesh<V>{
    //material:String,
    name:String,
    geometry_type:GeometryType,
    lods:Vec<LOD<V>>,
    //geometryType
    //vertexFormat:String,
    //lods:Lod<V>,
}

//impl<V> Mesh<V>{

pub struct VirtualMesh<'a>{
    pub name:String,
    pub full_semantics:String,
    pub lods:Vec<Option<&'a collada::Mesh>>,
    pub vertex_count_per_polygon:usize,
}
