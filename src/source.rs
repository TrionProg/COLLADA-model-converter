use collada;

use SemanticsSourceLayerType;

pub struct VirtualSource<'a>{
    vertex_layer:&'a collada::VertexLayer,
    is_index:bool,
    layers:Vec<VirtualSourceLayer<'a>>,
}

pub struct VirtualSourceLayer<'a>{
    layer:&'a collada::SourceLayer,
    output_type:SemanticsSourceLayerType,
}

impl VirtualSource
