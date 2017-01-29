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

use std::collections::btree_map::Entry::{Occupied, Vacant};

use LOD;
use collada;

use lexer::{Cursor,Lexeme};

pub enum GeometryType{
    Lines,
    Triangles,
}

/*
pub struct Mesh<V>{
    //material:String,
    name:String,
    geometry_type:GeometryType,
    lods:Vec<LOD<V>>,
    //geometryType
    //vertexFormat:String,
    //lods:Lod<V>,
}*/
/*
pub trait Mesh{
    fn read_layer(virtual_mesh_lod:&collada::Mesh, cursor:&mut Cursor) -> Result<(&collada::VertexLayer,bool,Vec<&SourceLayer>), Error>{
        let vertex_layer=match cursor.next(){
            Lexeme::EOF => break,
            Lexeme::String(ref layerName) => {
                let vertex_layer=match mesh.get(layerName){
                    Some(ref vl) => vl,
                    None => return Err( Error::Other(format!("Mesh \"{}\" has no vertex layer \"{}\"",&virtual_mesh_lod.name,layerName)) )б
                };

                vertex_layer
            },
            _=>return Err( Error::SemanticsParse(format!("Expected name of layer, but {} has been found", cursor.lex.print())) ),
        };

        let index=match cursor.next(){
            Lexeme::Ampersand => { cursor.next(); true},
            _ => false,
        };

        if cursor.lex!=Lexeme::Bracket {
            return Err( Error::SemanticsParse(format!("Expected (, but {} has been found", cursor.lex.print())) );
        }

        let mut source_layers=Vec::new();

        loop{
            match cursor.next(){
                Lexeme::String(ref source_layer_name) =>


    }
    fn construct(virtual_mesh:&VirtualMesh, outputSemantics:&String) -> Result<Box<Mesh>, Error>{
        for cM in virtual_mesh.lods.iter(){
            let virtual_mesh=cM.unwrap();

            let mut cursor=Cursor::new(outputSemantics.as_str());

            let mut layers=Vec::new();

            loop{
                match cursor.next(){
                    Lexeme::EOF => break,
                    Lexeme::String(ref layerName) => {
                        let vertex_layer=match mesh.get(layerName){
                            Some(ref vl) => vl,
                            None => return Err( Error::Other(format!("Mesh \"{}\" has no vertex layer \"{}\"",&virtual_mesh.name,layerName)) )б
                        };

                        layers.push( (vertex_layer,
*/

//impl<V> Mesh<V>{

use VirtualLOD;

pub struct VirtualMesh<'a>{
    pub name:String,
    pub full_semantics:String,
    pub lods:Vec<VirtualLOD<'a>>,
    pub vertex_count_per_polygon:usize,
}
