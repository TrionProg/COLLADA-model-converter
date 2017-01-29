use Error;
use collada;

use SemanticsSource;
use SemanticsSourceLayerType;

pub struct VirtualSource<'a>{
    pub vertex_layer:&'a collada::VertexLayer,
    pub is_index:bool,
    pub layers:Vec<VirtualSourceLayer<'a>>,
}

pub struct VirtualSourceLayer<'a>{
    pub layer:&'a collada::SourceLayer,
    pub layer_type:SemanticsSourceLayerType,
}

impl<'a> VirtualSource<'a>{
    pub fn parse(collada_mesh:&'a collada::Mesh, semantics_source:&SemanticsSource) -> Result<VirtualSource<'a>,Error>{
        let vertex_layer=match collada_mesh.vertex_layers.get(semantics_source.name){
            Some( vertex_layer_rc ) => vertex_layer_rc,
            None => return Err( Error::Other(format!("Mesh has no vertex layer \"{}\"", semantics_source.name)) ),
        };

        let mut layers=Vec::with_capacity(semantics_source.layers.len());

        for semantics_source_layer in semantics_source.layers.iter(){
            let source_layer=match vertex_layer.source.layers.get(semantics_source_layer.name){
                Some( source_layer ) => source_layer,
                None => return Err( Error::Other(format!("Source \"{}\" has no layer \"{}\"", &vertex_layer.source.id, semantics_source_layer.name)) ),
            };

            layers.push(VirtualSourceLayer{
                layer:source_layer,
                layer_type:semantics_source_layer.layer_type,
            });
        }

        Ok(VirtualSource{
            vertex_layer:vertex_layer,
            is_index:semantics_source.is_index,
            layers:layers,
        })
    }
}
