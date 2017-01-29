use Error;
use collada;

use VirtualSource;
use Semantics;

pub struct LOD<V>{
    pub distance:f32,
    pub dtt:V,
}

pub struct VirtualLOD<'a>{
    pub distance:f32,
    pub sources:Vec<VirtualSource<'a>>,
}

impl<'a> VirtualLOD<'a>{
    pub fn construct(collada_mesh:&'a collada::Mesh, distance:f32, semantics_text:&String) -> Result<VirtualLOD<'a>,Error>{
        let semantics=Semantics::parse(semantics_text)?;

        let mut sources=Vec::new();

        for semantics_source in semantics.sources.iter(){
            let virtual_source=VirtualSource::parse(collada_mesh, semantics_source)?;

            sources.push(virtual_source);
        }

        Ok(
            VirtualLOD{
                distance:distance,
                sources:sources,
            }
        )
    }
}
