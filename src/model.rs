use mesh::Mesh;
use std::path::Path;
use collada::document::ColladaDocument;

pub struct Model{
    name:String,
    meshes:Vec<Mesh>,
    //textures:Vec<Texture>,
}

impl Model{
    pub fn loadFromCollada(fileName:&Path) -> Result<Model,String>{
        let document=match ColladaDocument::from_path(fileName){
            Ok(d) => d,
            Err(e) => return Err(String::from(e)),
        };

        let meshes=match document.get_obj_set(){
            Some(objectSet) => {
                let mut meshes=Vec::new();

                for object in objectSet.objects.iter(){
                    println!("{}",object.name);
                }

                meshes
            },
            None => Vec::new(),
        };

        Ok(
            Model{
                name:String::from("model"),
                meshes:meshes,
            }
        )
    }
}
