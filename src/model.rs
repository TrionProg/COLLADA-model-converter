use Error;
use std::path::Path;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;

use collada;

//use Mesh;
use VirtualMesh;
use VirtualLOD;

pub struct Model{
    //meshes:HashMap<String, Mesh>,
}

//TODO add names for meshes to print them in error like GeomID.index

impl Model{
    pub fn from_collada(file_name:&Path) -> Result<Model,Error>{
        let document=match collada::Document::parse(file_name){
            Ok(d) => d,
            Err(e) => return Err(Error::ColladaError(e)),
        };

        let virtual_meshes=Self::generate_virtual_meshes(&document)?;

        for (name,vm) in virtual_meshes.iter(){
            println!("{} . {}",name,vm.lods.len());
            for lod in vm.lods.iter(){
                println!("d:{}",lod.distance);
                for source in lod.sources.iter(){
                    println!("{}",source.layers.len());
                }
            }
        }

        /*
        let mut lods_of_meshes=HashMap::new();

        for (distance,scene) in lods_of_scenes.iter(){
            let mut lods_of_meshes=HashMap::new();

            for (g,node) in scene.geometries.iter(){
                let geometry=&node.joined;

                for (i,mesh) in geometry.meshes.iter().enumerate(){
                    let mesh_name=if geometry.meshes.len()==1 {
                        node.name.clone()
                    }else{
                        match mesh.material{
                            Some( ref material ) =>
                                format!("{} m:{}",&node.name, material),
                            None =>
                                format!("{} #{}",&node.name, i),
                        }
                    };

                    match lods_of_meshes.entry(mesh_name){
                        Entry::Vacant(entry) => {
                            let mut lods=Vec::with_capacity(1);
                            lods.push(mesh);

                            entry.insert(lods);
                        },
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().push(mesh);
                        },
                    }
                }
            }
        }

        let mut meshes=HashMap::new();
        */

        Ok(Model{/*meshes:HashMap::new()*/})
    }

    pub fn generate_virtual_meshes<'a>(document:&'a collada::Document) -> Result<HashMap<String,VirtualMesh<'a>>,Error>{
        let (scene_name,scene)=match document.scenes.iter().next(){
            Some( (scene_name,scene) ) => (scene_name,scene),
            None => return Err(Error::Other( String::from("Collada document has no scenes") )),
        };

        let mut virtual_meshes=HashMap::new();

        for (node_name, node) in scene.geometries.iter(){
            let geometry=&node.joined;

            let (node_name, distance)=match geometry.name.find("_d:"){
                Some( pos ) => {
                    let (node_name, wast_and_distance)=geometry.name.split_at(pos);
                    let (wast,distance_str)=wast_and_distance.split_at("_d:".len());

                    let distance=match distance_str.parse::<f32>(){
                        Ok(d) => d,
                        Err(_) => return Err(Error::StringParseError( format!("Can not parse {} as f32",distance_str) )),
                    };

                    (String::from(node_name), distance)
                },
                None =>
                    (node_name.clone(),0.0),
            };

            for (i,mesh) in geometry.meshes.iter().enumerate(){
                let mesh_name=if geometry.meshes.len()==1 {
                    node_name.clone()
                }else{
                    match mesh.material{
                        Some( ref material_id ) =>
                            format!("{}_{}",node_name,material_id),
                        None =>
                            format!("{} #{}",node_name, i),
                    }
                };

                if mesh.polygons.len()==0 {
                    return Err(Error::Other( format!("Mesh {} of geometry\"{}\" has no polygons",i,geometry.name) ));
                }

                let vertex_count_per_polygon=mesh.polygons[0].vertices_count;

                for polygon in mesh.polygons.iter().skip(1){
                    if polygon.vertices_count!=vertex_count_per_polygon {
                        return Err( Error::Other(format!("Mesh \"{}\" expects {} vertices per polygon, but polygon with {} vertices has been found", &mesh_name, vertex_count_per_polygon, polygon.vertices_count)) );
                    }
                }

                let virtual_lod=VirtualLOD::construct(&mesh, distance, &String::from("VERTEX:(X:integer,Y:integer) NORMAL:(X:float,Y:float)"))?;

                match virtual_meshes.entry(mesh_name.clone()){
                    Entry::Vacant(entry) => {
                        let mut lods=Vec::with_capacity(1);
                        lods.push(virtual_lod);

                        entry.insert(
                            VirtualMesh{
                                name:mesh_name,
                                full_semantics:mesh.full_semantics.clone(),
                                lods:lods,
                                vertex_count_per_polygon:vertex_count_per_polygon,
                            }
                        );
                    },
                    Entry::Occupied(mut entry) =>
                        entry.get_mut().lods.push(virtual_lod),
                }
            }
        }

        for (_,virtual_mesh) in virtual_meshes.iter_mut(){
            virtual_mesh.lods.sort_by(|lod1,lod2| lod1.distance.partial_cmp(&lod2.distance).unwrap());

            if virtual_mesh.lods[0].distance!=0.0 {
                return Err( Error::Other( format!("Mesh \"{}\" must have LOD with 0 distance", virtual_mesh.name) ) );
            }

            for (lod1,lod2) in virtual_mesh.lods.iter().zip(virtual_mesh.lods.iter().skip(1)){
                if lod1.distance==lod2.distance {
                    return Err( Error::Other( format!("Mesh \"{}\" has LODS with same distance", virtual_mesh.name) ) );
                }
            }
        }

        Ok(virtual_meshes)
    }
}
