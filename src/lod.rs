use collada;

use VirtualSource;

pub struct LOD<V>{
    distance:f32,
    dtt:V,
}

pub struct VirtualLOD<'a>{
    sources:Vec<VirtualSource<'a>>,
}

impl<'a> VirtualLOD<'a>{
    pub fn construct(virtualMesh
