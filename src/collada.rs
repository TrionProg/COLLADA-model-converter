//use bincode::rustc_serialize::*;
use byteorder::{ByteOrder, LittleEndian};
use bincode_ext::byte_order::Le;

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};

use std::fs::File;
use std::io::{Write,BufReader};

use xml::reader::{EventReader, XmlEvent};

use mesh::Mesh;
use geometry::Geometry;

pub struct Parser (EventReader<BufReader<File>>);

fn checkCollada(parser:&mut EventReader<BufReader<File>>) -> bool {
    match parser.next(){
        Ok(XmlEvent::StartDocument{ .. } )=>{},
        _ => return false,
    }

    match parser.next(){
        Ok(XmlEvent::StartElement { name, .. }) => {
            if name.local_name.as_str()!="COLLADA" {
                return false;
            }

            match name.namespace{
                Some( ns ) => {
                    if ns.as_str()!="http://www.collada.org/2005/11/COLLADASchema"{
                        return false;
                    }
                },
                None => return false,
            }

            true
        },
        _=> false,
    }
}

pub fn skip( parser:&mut Parser, until:String ) -> Result<(),String>{
    let mut stack=Vec::with_capacity(8);
    stack.push( until );

    loop{
        match try!(parser.next()) {
            XmlEvent::StartElement { name, .. } => {
                stack.push(name.local_name);
            },
            XmlEvent::EndElement { name } => {
                let expected=stack.pop().unwrap();
                if name.local_name!=expected {
                    return Err( format!("Expected {}, but {} have been found", expected, name.local_name) );
                }

                if stack.len()==0 {
                    return Ok(());
                }
            },
            _ => {},
        }
    }
}

pub fn waitStartTag( parser:&mut Parser, tagName:&'static str ) -> Result<XmlEvent, String>{
    loop{
        let tag=try!(parser.next());

        let found=match tag {
            XmlEvent::StartElement { ref name, .. } => {
                if name.local_name.as_str() != tagName {
                    return Err( format!("Expected \"{}\" tag", tagName) );
                }

                true
            },
            XmlEvent::Whitespace ( _ ) | XmlEvent::Comment ( _ ) => false,
            _ => return Err( format!("Expected \"{}\" tag", tagName) ),
        };

        if found {
            return Ok(tag);
        }
    }
}

pub fn waitEndTag( parser:&mut Parser, tagName:&'static str ) -> Result<(), String>{
    loop{
        match try!(parser.next()) {
            XmlEvent::EndElement { name } => {
                if name.local_name.as_str() != tagName {
                    return Err( format!("Expected \"{}\" end tag", tagName) );
                }

                return Ok(());
            },
            XmlEvent::Whitespace ( _ ) | XmlEvent::Comment ( _ ) => {},
            _ => return Err( format!("Expected \"{}\" end tag", tagName) ),
        }
    }
}

impl Parser{
    pub fn next(&mut self) -> Result<XmlEvent, String>{
        match self.0.next(){
            Ok ( e ) => {
                match e {
                    XmlEvent::EndDocument => Err( String::from("Unexpected end of file") ),
                    _ => Ok( e ),
                }
            },
            Err( e ) => Err(format!("Can not parse:{}",e) ),
        }
    }
}


pub fn convertModel( inFileName:String, outFileName:String ) -> Result<(), String>{
    let file = match File::open(&inFileName){
        Ok ( f ) => f,
        Err( e ) => return Err( format!("Can not read file \"{}\" : {:?}", inFileName, e) ),
    };

    let file = BufReader::new(file);

    let mut reader = EventReader::new(file);

    if !checkCollada(&mut reader){
        return Err( format!("File \"{}\" is no collada file",inFileName) );
    }

    let mut parser = Parser(reader);

    let mut geometries=Vec::new();

    loop{
        match try!(parser.next()){
            XmlEvent::StartElement { name, .. } => {
                match name.local_name.as_str(){
                    "library_geometries" => {
                        loop{
                            match try!(parser.next()){
                                XmlEvent::StartElement { name, attributes , .. } => {
                                    if name.local_name.as_str()=="geometry" {
                                        let mut specifiedMeshName=None;
                                        let mut specifiedMeshID=None;

                                        for attr in attributes{
                                            match attr.name.local_name.as_str(){
                                                "id" => specifiedMeshID=Some(attr.value),
                                                "name" => specifiedMeshName=Some(attr.value),
                                                _ => {},
                                            }
                                        }

                                        let meshName=match specifiedMeshName{
                                            Some (n) => n,
                                            None => return Err( String::from("mesh name has not been specified") ),
                                        };

                                        let meshID=match specifiedMeshID{
                                            Some (id) => id,
                                            None => return Err( String::from("mesh id has not been specified") ),
                                        };


                                        println!("geom {} {}",&meshID, &meshName);
                                        //try!( skip( &mut parser, name.local_name) );
                                        let parsedGeometryes = match Geometry::readCollada(&mut parser, &meshID) {
                                            Ok ( g ) => g,
                                            Err( e ) => return Err( format!("Geometry id : {} {}", meshID, e) ),
                                        };

                                        for geometry in parsedGeometryes{
                                            geometries.push(geometry);
                                        }

                                        //geometries.append(parsedGeometryes);

                                        loop{
                                            match try!(parser.next()) {
                                                XmlEvent::EndElement { name } => {
                                                    if name.local_name.as_str()!="geometry" {
                                                        return Err( String::from("Expected geometry end tag") );
                                                    }

                                                    break;
                                                },
                                                XmlEvent::Whitespace ( _ ) | XmlEvent::Comment ( _ ) => {},
                                                _=> return Err( String::from("Expected geometry end tag") ),
                                            }
                                        }
                                    }
                                },
                                XmlEvent::EndElement { name } => {
                                    if name.local_name.as_str()=="library_geometries" {
                                        break;
                                    }else{
                                        return Err( format!("Unexpected end tag \"{}\"", name.local_name) );
                                    }
                                },
                                _ => {},
                            }
                        }
                    },
                    _ => try!( skip( &mut parser, name.local_name) ),
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name.as_str()=="COLLADA" {
                    break;
                }else{
                    return Err( format!("Unexpected end tag \"{}\"", name.local_name) );
                }
            },
            _ => {},
        }
    }

    let mut outFile = match File::create(&outFileName){
        Ok ( f ) => f,
        Err( e ) => return Err( format!("Can not write file \"{}\" : {:?}", outFileName, e) ),
    };

    use byteorder::{LittleEndian, WriteBytesExt};
    let mut lengthBuf=vec![];

    lengthBuf.write_u64::<LittleEndian>(geometries.len() as u64);
    //outFile.write(&lengthBuf[..]);

    for geometry in geometries.iter(){
        let mut lengthBuf=vec![];
        lengthBuf.write_u64::<LittleEndian>((geometry.polygonsCount*3) as u64);
        outFile.write(&lengthBuf[..]);

        outFile.write(&geometry.data[..]);
    }

    Ok(())
}
