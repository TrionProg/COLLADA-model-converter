/*
extern crate xml;
extern crate rustc_serialize;
extern crate bincode;
extern crate bincode_ext;
extern crate byteorder;

use std::env;

mod collada;
mod geometry;
mod mesh;
mod source;

fn process() -> Result<(), String>{
    let mut argsIter=env::args();

    argsIter.next();

    match argsIter.next(){
        None => return Err( String::from("expected action : convert") ),
        Some( action ) => {
            match action.as_str() {
                "convert" => {
                    let inFileName=match argsIter.next(){
                        Some( fileName ) => fileName.clone(),
                        None => return Err( String::from("expected name of file to convert") ),
                    };

                    let mut specifiedOutFileName=None;

                    loop{
                        match argsIter.next(){
                            Some( option ) => {
                                match option.as_str(){
                                    "to" => {
                                        match argsIter.next(){
                                            Some( outFileName ) => specifiedOutFileName=Some( outFileName.clone() ),
                                            None => return Err( String::from("expected name of file") ),
                                        }
                                    },
                                    _ => return Err( format!("Unknown option {}", option) ),
                                }
                            },
                            None => break,
                        }
                    }

                    let outFileName=match specifiedOutFileName{
                        Some( fileName ) => fileName,
                        None => {
                            match inFileName.rfind('.'){
                                Some( pos ) => {
                                    let (fileName, extension)=inFileName.split_at(pos);

                                    format!("{}.pmdl",fileName)
                                },
                                None => format!("{}.pmdl", &inFileName)
                            }
                        }
                    };

                    try!(collada::convertModel(inFileName, outFileName));
                },
                _ =>return Err( String::from("action should be one of \"convert\"") ),
            }
        },
    }

    Ok(())
}


fn main() {
    match process(){
        Ok ( _ ) => {},
        Err( e ) => {
            println!("{}",e);
            return;
        }
    }

    /*
    let file = File::open("track.dae").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        //println!("{:?}", e);
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", depth, name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", depth, name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    */
}
*/
extern crate collada;

mod lexer;

mod error;
pub use error::Error;

mod semantics;
pub use semantics::{Semantics,SemanticsSource,SemanticsSourceLayer,SemanticsSourceLayerType};

mod source;
pub use source::{VirtualSource,VirtualSourceLayer};

mod lod;
pub use lod::{LOD,VirtualLOD};

mod mesh;
pub use mesh::VirtualMesh;//{Mesh,GeometryType};

mod model;
pub use model::Model;

use std::path::Path;

//TODO:load from collada as new implementations;

fn main(){
    let model=match Model::from_collada(&Path::new("a2.dae")){
        Ok(d) => d,
        Err(e) => panic!("{}",e),
    };

    /*
    document.print_tree();

    let scene=document.scenes.get("Scene").unwrap();
    let node=scene.geometries.get("body").unwrap();
    let geometry=&node.joined;
    let mesh=&geometry.meshes[0];
    println!("{}",mesh.full_semantics);
    let polygon=&mesh.polygons[3];
    let position=mesh.vertex_layers.get("VERTEX").unwrap();
    let y_source_layer=position.source.layers.get("Y").unwrap();
    let source_data=match *y_source_layer {
        collada::SourceLayer::Float(ref data) => data,
        _ => panic!("we expect only float"),
    };
    let vertex_index=polygon.first_vertex_index+1;
    println!("Y coord is {}",source_data[position.indexes[vertex_index]]);
    */

}
