use Error;
use std::path::Path;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use collada::{Document,Geometry};

use Mesh;

pub struct Model{
    meshes:HashMap<String, Mesh>,
}

impl Model{
    pub fn from_collada(file_name:&Path) -> Result<Model,Error>{
        let document=match Document::parse(file_name){
            Ok(d) => d,
            Err(e) => return Err(Error::ColladaError(e)),
        };

        for (_,scene) in document.scenes.iter(){
            let mut lods_of_meshes=HashMap::new();

            for (g,node) in scene.geometries.iter(){
                let geometry=&node.joined;

                let (node_name, distance)=match node.name.find("_d:"){
                    Some( pos ) => {
                        let (node_name, wast_and_distance)=node.name.split_at(pos);
                        let (wast,distance_str)=wast_and_distance.split_at("_d:".len());

                        let distance=match distance_str.parse::<f32>(){
                            Ok(d) => d,
                            Err(_) => return Err(Error::StringParseError( format!("Can not parse {} as f32",distance_str) )),
                        };

                        (String::from(node_name), Some(distance))
                    },
                    None =>
                        (node.name.clone(), None)
                };

                for (i,mesh) in geometry.meshes.iter().enumerate(){
                    let mesh_name=if geometry.meshes.len()==1 {
                        node_name.clone()
                    }else{
                        match mesh.material{
                            Some( ref material ) =>
                                format!("{} m:{}",&node_name, material),
                            None =>
                                format!("{} #{}",&node_name, i),
                        }
                    };

                    match lods_of_meshes.entry(mesh_name){
                        Entry::Vacant(entry) => {
                            let mut lods=Vec::with_capacity(1);
                            lods.push((distance, mesh));

                            entry.insert(lods);
                        },
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().push((distance, mesh));
                        },
                    }
                }
            }

            for (_,mut lods) in lods_of_meshes.iter_mut(){
                lods.sort_by(|lod1, lod2| lod1.0.partial_cmp(&lod2.0).unwrap());
            }

            let mut meshes=HashMap::new();

            
        }

        Ok(Model{meshes:HashMap::new()})
    }
}
