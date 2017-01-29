use collada;
use lexer::{Cursor,Lexeme};

pub struct VirtualLayer<'a>{
    vertex_layer:&'a collada::VertexLayer,
    is_index:bool,
    source_layers:Vec<&'a collada::SourceLayer>,
}
/*
impl VirtualLayer{
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
*/
