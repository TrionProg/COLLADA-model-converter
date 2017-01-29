use Error;
use std::path::Path;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;

use collada;

//use Mesh;
use mesh::VirtualMesh;

pub struct Model{
    //meshes:HashMap<String, Mesh>,
}

impl Model{
    pub fn from_collada(file_name:&Path) -> Result<Model,Error>{
        let document=match Document::parse(file_name){
            Ok(d) => d,
            Err(e) => return Err(Error::ColladaError(e)),
        };

        let lods_from_scenes=Self::generate_lods_from_scenes(&document)?;
        let virtual_meshes=Self::generate_virtual_meshes(&lods_from_scenes)?;

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

    pub fn generate_virtual_meshes(document:&collada::Document

    pub fn generate_lods_from_scenes(document:&Document) -> Result<Vec<(f32,&Rc<Scene>)>,Error>{
        let mut lods_from_scenes=Vec::with_capacity(document.scenes.len());
        let mut is_lod_without_distance=false;

        println!("{}",document.scenes.len());

        for (_,scene) in document.scenes.iter(){
            let distance=match scene.name.find("_d:"){
                Some( pos ) => {
                    let (node_name, wast_and_distance)=scene.name.split_at(pos);
                    let (wast,distance_str)=wast_and_distance.split_at("_d:".len());

                    match distance_str.parse::<f32>(){
                        Ok(d) => d,
                        Err(_) => return Err(Error::StringParseError( format!("Can not parse {} as f32",distance_str) )),
                    }
                },
                None => {
                    if is_lod_without_distance {
                        return Err(Error::Other( format!("Scene \"{}\" has no distance it should be like \"{}_200\"",&scene.name,&scene.name) ));
                    }

                    is_lod_without_distance=true;

                    0.0
                },
            };

            lods_from_scenes.push((distance, scene));
        }

        if lods_from_scenes.len()==0 {
            return Err( Error::Other( String::from("This model has no scenes") ) );
        }

        lods_from_scenes.sort_by(|lod1,lod2| lod1.0.partial_cmp(&lod2.0).unwrap());

        if lods_from_scenes[0].0!=0.0 {
            return Err( Error::Other( String::from("One scene must have 0 distance") ) );
        }

        for (lod1,lod2) in lods_from_scenes.iter().zip(lods_from_scenes.iter().skip(1)){
            if lod1.0==lod2.0 {
                return Err( Error::Other( format!("Scene \"{}\" has same distance loke \"{}\"", lod1.1.name, lod2.1.name) ) );
            }
        }

        Ok(lods_from_scenes)
    }

    pub fn generate_virtual_meshes<'a>(lods_from_scenes:&'a Vec<(f32,&Rc<Scene>)>) -> Result<HashMap<String,VirtualMesh<'a>>,Error>{
        let mut virtual_meshes=HashMap::new();

        let scene=&lods_from_scenes[0].1;

        for (_,node) in scene.geometries.iter(){
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

                if mesh.polygons.len()==0 {
                    return Err( Error::Other(format!("Mesh \"{}\" has no polygons",&mesh_name)) );
                }

                let vertex_count_per_polygon=mesh.polygons[0].vertices_count;

                for polygon in mesh.polygons.iter().skip(1){
                    if polygon.vertices_count!=vertex_count_per_polygon {
                        return Err( Error::Other(format!("Mesh \"{}\" expects {} vertices per polygon, but polygon with {} vertices has been found", &mesh_name, vertex_count_per_polygon, polygon.vertices_count)) );
                    }
                }

                match virtual_meshes.entry(mesh_name.clone()){
                    Entry::Vacant(entry) => {
                        let mut lods=Vec::with_capacity(lods_from_scenes.len());
                        lods.resize(lods_from_scenes.len(),None);

                        lods[0]=Some(mesh);

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
                        return Err( Error::Other(format!("Mesh \"{}\" already exists!", &mesh_name)) ),
                }
            }
        }

        for (lod_index,&(distance,scene)) in lods_from_scenes.iter().enumerate().skip(1){
            println!("{}",distance);
            for (_,node) in scene.geometries.iter(){
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

                    match virtual_meshes.get_mut(&mesh_name){
                        Some( ref mut virtual_mesh ) => {
                            if virtual_mesh.full_semantics.as_str()!=mesh.full_semantics.as_str() {
                                return Err( Error::Other(format!("Mesh \"{}\" expects semantics \"{}\", but LOD {} has semantics \"{}\"",
                                    virtual_mesh.name,
                                    virtual_mesh.full_semantics,
                                    distance,
                                    mesh.full_semantics,
                                )));
                            }

                            for polygon in mesh.polygons.iter().skip(1){
                                if polygon.vertices_count!=virtual_mesh.vertex_count_per_polygon {
                                    return Err( Error::Other(format!("Mesh \"{}\" expects {} vertices per polygon, but polygon with {} vertices has been found",virtual_mesh.name, virtual_mesh.vertex_count_per_polygon, polygon.vertices_count)) );
                                }
                            }

                            match virtual_mesh.lods[lod_index] {
                                Some(_) => return Err( Error::Other(format!("LOD {} of mesh \"{}\" already exists", distance, mesh_name)) ),
                                None => {},
                            }

                            virtual_mesh.lods[lod_index]=Some(mesh);
                        },
                        None =>
                            return Err( Error::Other(format!("Unknown mesh \"{}\" of LOD {}", mesh_name, distance)) ),
                    }
                }
            }
        }

        Ok(virtual_meshes)
    }
}
