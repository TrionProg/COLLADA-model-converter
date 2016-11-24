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
