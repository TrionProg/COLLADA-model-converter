use xml::reader::{EventReader, XmlEvent};
use collada::{Parser,skip,waitStartTag,waitEndTag};

pub enum Source{
    XYZ ( String, Vec<f32> ),
    UV( String, usize, Vec<f32> ),
    Index( Vec<usize> ),
}

impl Source{
    pub fn readCollada( parser:&mut Parser, sourceType:&str ) -> Result<Source, String> {
        let mut data=None;
        let mut count=None;
        let mut stride=None;
        let mut params=Vec::with_capacity(4);

        loop{
            match try!(parser.next()) {
                XmlEvent::StartElement { name, attributes, .. } => {
                    let startTagName=name;
                    match startTagName.local_name.as_str(){
                        "float_array" => {
                            let mut specifiedCount=None;
                            let mut specifiedID=None;

                            for attr in attributes {
                                match attr.name.local_name.as_str(){
                                    "count" => {
                                        let count=match attr.value.parse::<usize>(){
                                            Ok ( c ) => c,
                                            Err( e ) => return Err( format!("Can not parse count \"{}\"", &attr.value)),
                                        };

                                        specifiedCount=Some(count);
                                    },
                                    "id" => specifiedID=Some(attr.value),
                                    _ => {},
                                }
                            }

                            let id=match specifiedID{
                                Some( c ) => c,
                                None => return Err( String::from("ID does not exists")),
                            };

                            let count=match specifiedCount{
                                Some( c ) => c,
                                None => return Err( String::from("Count does not exists")),
                            };

                            match try!(parser.next()) {
                                XmlEvent::Characters ( d ) => {
                                    match data{
                                        Some( .. ) => return Err( String::from("Dublicate data in same source")),
                                        None => data=Some( (id,count,d) ),
                                    }
                                },
                                _ => return Err( String::from("Mesh data does not exists")),
                            }

                            try!(waitEndTag( parser, "float_array" ));
                        },
                        "technique_common" => {
                            let accessorTag=try!(waitStartTag( parser, "accessor" ));

                            match accessorTag {
                                XmlEvent::StartElement { name, attributes, .. } => {
                                    if name.local_name.as_str()!="accessor" {
                                        return Err( String::from("Expected accessor tag") );
                                    }

                                    let mut specifiedCount=None;
                                    let mut specifiedStride=None;

                                    for attr in attributes {
                                        match attr.name.local_name.as_str(){
                                            "count" => {
                                                let count=match attr.value.parse::<usize>(){
                                                    Ok ( c ) => c,
                                                    Err( e ) => return Err( format!("Can not parse count \"{}\"", &attr.value)),
                                                };

                                                specifiedCount=Some(count);
                                            },
                                            "source" => {
                                                let dataID=attr.value.trim_left_matches('#');

                                                match data {
                                                    Some( (ref id, _ , _) ) => {
                                                        if id.as_str()!=dataID {
                                                            return Err( format!("Source data id({}) and accessor id({}) dismatch", id, dataID) );
                                                        }
                                                    },
                                                    None =>
                                                        return Err( format!("Source data \"{}\" has not been specified yet", dataID) ),
                                                }
                                            },
                                            "stride" => {
                                                let stride=match attr.value.parse::<usize>(){
                                                    Ok ( c ) => c,
                                                    Err( e ) => return Err( format!("Can not parse stride \"{}\"", &attr.value) ),
                                                };

                                                specifiedStride=Some(stride);
                                            },
                                            _ => {},
                                        }
                                    }

                                    match specifiedCount{
                                        Some( c ) => count=Some(c),
                                        None => return Err( String::from("Count does not exists") ),
                                    }

                                    match specifiedStride{
                                        Some( s ) => stride=Some(s),
                                        None => return Err( String::from("Stride does not exists") ),
                                    }
                                }
                                _ => panic!("Unreachable"),
                            }

                            loop{
                                match try!(parser.next()) {
                                    XmlEvent::StartElement { name, attributes, .. } => {
                                        if name.local_name.as_str()=="param" {
                                            let mut specifiedName=None;
                                            let mut specifiedType=None;

                                            for attr in attributes {
                                                match attr.name.local_name.as_str(){
                                                    "name" => specifiedName=Some(attr.value),
                                                    "type" => specifiedType=Some(attr.value),
                                                    _ => {},
                                                }
                                            }

                                            let paramName=match specifiedName{
                                                Some( n ) => n,
                                                None => return Err( String::from("Param name does not exists") ),
                                            };

                                            let paramType=match specifiedType{
                                                Some( t ) => t,
                                                None => return Err( String::from("Param type does not exists") ),
                                            };

                                            params.push( (paramName, paramType) );

                                            try!(waitEndTag( parser, "param" ));
                                        }else{
                                            try!( skip( parser, name.local_name) );
                                        }
                                    },
                                    XmlEvent::EndElement { name } => {
                                        if name.local_name.as_str()=="accessor" {
                                            break;
                                        }else{
                                            return Err( String::from("Expected accessor end tag") );
                                        }
                                    },
                                    _ => {},
                                }
                            }

                            try!(waitEndTag( parser, "technique_common" ));
                        }
                        _ => try!( skip( parser, startTagName.local_name) ),
                    }
                },
                XmlEvent::EndElement { name } => {
                    match name.local_name.as_str(){
                        "source" => break,
                        _ => return Err( format!("Unexpected end tag \"{}\"", name.local_name) ),
                    }
                },
                _ => {}
            }
        }

        let (dataID, dataCount, data)=match data{
            Some( d ) => d,
            None => return Err( String::from("Source has no data") ),
        };

        let count=match count{
            Some( c ) => c,
            None => return Err( String::from("Count of elements of data has not been specified") ),
        };

        let stride=match stride{
            Some( s ) => s,
            None => return Err( String::from("Stride of elements of data has not been specified") ),
        };

        if count*stride!=dataCount {
            return Err( String::from("count*stride!=dataCount") );
        }

        if stride!=params.len() {
            return Err( String::from("stride!=params number") );
        }

        if sourceType=="positions" || sourceType=="normals"{ //XYZ
            if params.len()!=3 {
                return Err( format!("{} source expects 3 coordinates(XYZ)", sourceType) );
            }

            for &( _ , ref paramType ) in params.iter(){
                if paramType.as_str()!="float" {
                    return Err( format!("{} Source must use float data type, found \"{}\"", sourceType, paramType) );
                }
            }

            let mut buffer=Vec::with_capacity(dataCount);

            let mut index=0;

            for v in data.split(' '){
                if v!=""{
                    if index>=dataCount {
                        return Err( String::from("Mesh data has more length than specified") );
                    }

                    buffer.push(
                        match v.parse::<f32>(){
                            Ok ( v ) => v,
                            Err( e ) => return Err( format!("Can not parse mesh data {}", v) ),
                        }
                    );

                    index+=1;
                }
            }

            Ok( Source::XYZ( String::from(sourceType), buffer ) )
        }else if sourceType.starts_with("map-0") { //UV
            if params.len()!=2 {
                return Err( format!("{} source expects 2 coordinates(UV)", sourceType) );
            }

            for &( _ , ref paramType ) in params.iter(){
                if paramType.as_str()!="float" {
                    return Err( format!("{} Source must use float data type, found \"{}\"", sourceType, paramType) );
                }
            }

            let(_, tiStr)=sourceType.split_at("map".len()+1);

            let ti=match tiStr.parse::<usize>(){
                Ok ( l ) => l,
                Err( _ ) => return Err( String::from("Can not parse texture index") ),
            };

            if ti!=0 {
                return Err( format!("Multitexturing is not supported : {}", sourceType) );
            }

            let mut buffer=Vec::with_capacity(dataCount);

            let mut index=0;

            for v in data.split(' '){
                if v!=""{
                    if index>=dataCount {
                        return Err( String::from("Mesh data has more length than specified") );
                    }

                    buffer.push(
                        match v.parse::<f32>(){
                            Ok ( v ) => v,
                            Err( e ) => return Err( format!("Can not parse mesh data {}", v) ),
                        }
                    );

                    index+=1;
                }
            }

            Ok( Source::UV( String::from("map"), ti, buffer ) )
        }else{
            Err( format!("Unknown source type : {}", sourceType) )
        }
    }
}
