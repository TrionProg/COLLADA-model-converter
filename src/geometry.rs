use xml::reader::{EventReader, XmlEvent};
use collada::{Parser,skip,waitStartTag,waitEndTag};

use std::collections::BTreeMap;
use std::collections::btree_map::Entry as BTreeMapEntry;

use source::{SourceData,Source};
use mesh::{GeometryType, Mesh, VertexP3, VertexP3N3, VertexP3N3T0C2};

use byteorder::{LittleEndian, WriteBytesExt};

pub enum ConvertTo{
    to_f32,
    to_i32,
}

pub struct Geometry{
    material:Option<String>,
    data:Vec<u8>,
    polygonsCount:usize,
}

impl Geometry{
    pub fn readCollada( parser:&mut Parser, meshID:&String ) -> Result<Vec<Geometry>, String> {
        try!(waitStartTag(parser, "mesh" ));

        let mut sources=BTreeMap::new();
        let mut geometries=Vec::new();

        loop {
            match try!(parser.next()) {
                XmlEvent::StartElement { name, attributes, .. } => {
                    match name.local_name.as_str(){
                        "source" => {
                            let mut specifiedSourceID=None;

                            for attr in attributes {
                                match attr.name.local_name.as_str(){
                                    "id" => specifiedSourceID=Some(attr.value),
                                    _ => {},
                                }
                            }

                            let sourceID=match specifiedSourceID{
                                Some( id ) => id,
                                None => return Err( String::from("Source of mesh needs id")),
                            };

                            if !sourceID.starts_with(meshID.as_str()) {
                                return Err( format!("Source ID shoult begin with ID of mesh : {} / {}", meshID, sourceID) );
                            }

                            let(_, sourceType)=sourceID.split_at(meshID.len()+1);

                            let source=match Source::readCollada(parser, sourceType){
                                Ok ( s ) => s,
                                Err( e ) => return Err( format!("Source id: {} error : {}",sourceID, e) ),
                            };

                            match sources.entry( sourceID.clone() ){
                                BTreeMapEntry::Occupied ( _ ) => return Err( format!("Duplicate source id {}", &sourceID) ),
                                BTreeMapEntry::Vacant( e ) => {e.insert( source );},
                            }
                        },
                        "vertices" => {
                            let mut specifiedID=None;

                            for attr in attributes {
                                match attr.name.local_name.as_str(){
                                    "id" => specifiedID=Some(attr.value),
                                    _ => {},
                                }
                            }

                            let newID=match specifiedID{
                                Some( c ) => c,
                                None => return Err( String::from("ID does not exists")),
                            };

                            loop{
                                match try!(parser.next()) {
                                    XmlEvent::StartElement { name, attributes, .. } => {
                                        match name.local_name.as_str(){
                                            "input" => {
                                                let mut specifiedSourceID=None;

                                                for attr in attributes {
                                                    match attr.name.local_name.as_str(){
                                                        "source" => specifiedSourceID=Some(attr.value),
                                                        _ => {},
                                                    }
                                                }

                                                let sourceID=match specifiedSourceID{
                                                    Some( c ) => c,
                                                    None => return Err( String::from("input tag:Source does not exists")),
                                                };

                                                let sourceID=sourceID.trim_matches('#');

                                                let source=match sources.remove( sourceID ){
                                                    Some( s ) => s,
                                                    None => return Err( format!("Source with id \"{}\" does not exists", sourceID) ),
                                                };

                                                match sources.entry( newID.clone() ){
                                                    BTreeMapEntry::Occupied ( _ ) => return Err( format!("input tag:Source \"{}\" already exists", &newID) ),
                                                    BTreeMapEntry::Vacant( e ) => {e.insert( source );},
                                                }

                                                try!(waitEndTag( parser, "input" ));
                                            },
                                            _ => try!( skip( parser, name.local_name) ),
                                        }

                                    },
                                    XmlEvent::EndElement { name } => {
                                        match name.local_name.as_str() {
                                            "vertices" => break,
                                            _ => return Err( String::from("expected vertices end tag") ),
                                        }
                                    },
                                    _ => {},
                                }
                            }
                        },
                        "polylist" => {
                            let mut specifiedCount=None;
                            let mut specifiedMaterial=None;

                            for attr in attributes {
                                match attr.name.local_name.as_str(){
                                    "count" => {
                                        let count=match attr.value.parse::<usize>(){
                                            Ok ( c ) => c,
                                            Err( e ) => return Err( format!("Can not parse count \"{}\"", &attr.value)),
                                        };

                                        specifiedCount=Some(count);
                                    },
                                    "material" => specifiedMaterial=Some(attr.value),
                                    _ => {},
                                }
                            }

                            let polygonsCount=match specifiedCount{
                                Some( c ) => c,
                                None => return Err( String::from("Count does not exists")),
                            };

                            let mut sourcesList=Vec::new();
                            let mut specifiedPolygonIndexes=None;

                            let mut isVertex=false;
                            let mut isNormal=false;
                            let mut isUV=false;

                            let mut vertexSemantics=String::with_capacity(32);

                            loop{
                                match try!(parser.next()){
                                    XmlEvent::StartElement { name, attributes, .. } => {
                                        match name.local_name.as_str(){
                                            "input" => {
                                                let mut specifiedSemantic=None;
                                                let mut specifiedSourceID=None;

                                                for attr in attributes{
                                                    match attr.name.local_name.as_str(){
                                                        "semantic" => specifiedSemantic=Some(attr.value),
                                                        "source" => specifiedSourceID=Some(attr.value),
                                                        _ => {},
                                                    }
                                                }

                                                let semantic=match specifiedSemantic{
                                                    Some( s ) => s,
                                                    None => return Err( String::from("Semantic does not exists")),
                                                };

                                                let sourceID=match specifiedSourceID{
                                                    Some( c ) => c,
                                                    None => return Err( String::from("Source does not exists")),
                                                };

                                                let sourceID=sourceID.trim_left_matches('#');


                                                let source=match sources.get(sourceID){
                                                    Some( ref src ) => &sources[sourceID],
                                                    None => return Err( format!("Source \"{}\" does not exists",sourceID) ),
                                                };

                                                match semantic.as_str(){
                                                    "VERTEX" | "NORMAL" | "TEXCOORD" => {
                                                        vertexSemantics.push_str(&semantic);
                                                        vertexSemantics.push('(');

                                                        for sourceData in source.data.iter() {
                                                            match *sourceData {
                                                                SourceData::Float( ref paramName, _ ) => {
                                                                    vertexSemantics.push('f');
                                                                    vertexSemantics.push_str(paramName);
                                                                },
                                                                SourceData::Int( ref paramName, _ ) => {
                                                                    vertexSemantics.push('i');
                                                                    vertexSemantics.push_str(paramName);
                                                                },
                                                            }
                                                        }

                                                        vertexSemantics.push_str(") ");
                                                        sourcesList.push( (source, false) );
                                                    },
                                                    _ => {
                                                        println!("Source \"{}\" is ignored", semantic);

                                                        sourcesList.push( (source, true) );
                                                    }
                                                }

                                                try!(waitEndTag( parser, "input" ));
                                            },
                                            "vcount" => {
                                                let vertexesPerPolygon=match try!(parser.next()) {
                                                    XmlEvent::Characters ( vpp ) => vpp,
                                                    _ => return Err( String::from("<vcount> content does not exists")),
                                                };

                                                let mut index=0;

                                                for v in vertexesPerPolygon.split(' '){
                                                    if v!=""{
                                                        if index>=polygonsCount {
                                                            return Err( String::from("<vcount> content has more polygons than specified") );
                                                        }

                                                        match v.parse::<usize>(){
                                                            Ok( vcount ) => {
                                                                if vcount>3 {
                                                                    return Err( format!("Mesh has polygon with {} vertices, but only triangles are allowed. You need triangulate you mesh( Alt+T in edit mode for blender )", vcount) );
                                                                }else if vcount!=3 {
                                                                    return Err( format!("Mesh can contain only triangles, but polygon with {} vertices has been found", vcount) );
                                                                }
                                                            },
                                                            Err( _ ) => return Err( format!("Can not parse vertex count for polygon \"{}\"",v) ),
                                                        }

                                                        index+=1;
                                                    }
                                                }

                                                try!(waitEndTag( parser, "vcount" ));
                                            },
                                            "p" => {
                                                let indexesPerPolygon=match try!(parser.next()) {
                                                    XmlEvent::Characters ( ipp ) => ipp,
                                                    _ => return Err( String::from("<p> content does not exists")),
                                                };

                                                let mut buffer=Vec::with_capacity(polygonsCount*9);

                                                for v in indexesPerPolygon.split(' '){
                                                    if v!=""{
                                                        match v.parse::<usize>(){
                                                            Ok( index ) => {
                                                                buffer.push(index);
                                                            },
                                                            Err( _ ) => return Err( format!("Can not parse indexes polygon \"{}\"",v) ),
                                                        }
                                                    }
                                                }

                                                specifiedPolygonIndexes=Some(buffer);

                                                try!(waitEndTag( parser, "p" ));
                                            },
                                            _ => try!( skip( parser, name.local_name) ),
                                        }
                                    },
                                    XmlEvent::EndElement { name } => {
                                        match name.local_name.as_str(){
                                            "polylist" => break,
                                            _ => return Err( format!("Unexpected end tag \"{}\"", name.local_name) ),
                                        }
                                    },
                                    _ => {},
                                }
                            }

                            let polygonIndexes = match specifiedPolygonIndexes{
                                Some ( pi ) => pi,
                                None => return Err( String::from("Polygon indexes has not been specified") ),
                            };

                            let vertexSemantics=vertexSemantics.trim_matches(' ');

                            println!("{}",vertexSemantics);

                            let convertTOSemantic=match vertexSemantics{
                                "VERTEX(fXfYfZ) NORMAL(fXfYfZ) TEXCOORD(fSfT)" => "f32 f32 f32 f32 f32 f32 f32 f32",
                                _ => return Err( format!("Unknown vertex semantic \"{}\"", vertexSemantics) ),
                            };

                            let mut convertTOSemanticIter=convertTOSemantic.split(' ');

                            let mut convertAsList=Vec::new();
                            for &(source, ignore) in sourcesList.iter() {
                                if ignore {
                                    for sourceData in source.data.iter() {
                                        convertAsList.push(None);
                                    }
                                }else{
                                    for sourceData in source.data.iter() {
                                        match convertTOSemanticIter.next() {
                                            Some( t ) => {
                                                match t{
                                                    "f32" => convertAsList.push( Some((sourceData, ConvertTo::to_f32)) ),
                                                    "i32" => convertAsList.push( Some((sourceData, ConvertTo::to_i32)) ),
                                                    _ => return Err( String::from("tmperr") ),
                                                }
                                            },
                                            None => {},
                                        }
                                    }
                                }
                            }

                            for convertAs in convertAsList.iter(){
                                match *convertAs {
                                    Some( ( _ , ref convertTo ) ) => {
                                        match *convertTo {
                                            ConvertTo::to_f32 => println!("to f32"),
                                            ConvertTo::to_i32 => println!("to i32"),
                                        }
                                    },
                                    None => println!("ignore"),
                                }
                            }

                            let geometry=Geometry::constructTriangles(convertAsList, polygonIndexes, polygonsCount)?;

                            geometries.push( geometry );
                        },
                        _ => skip( parser, name.local_name)?,
                    }
                },
                XmlEvent::EndElement { name } => {
                    match name.local_name.as_str(){
                        "mesh" => break,
                        _ => return Err( format!("Unexpected end tag \"{}\"", name.local_name) ),
                    }
                },
                _ => {},
            }
        }

        Ok(geometries)
    }

    pub fn constructTriangles(convertAsList:Vec< Option<(&SourceData, ConvertTo)> >, polygonIndexes:Vec<usize>, polygonsCount:usize) -> Result<Geometry, String>{
        if convertAsList.len()*polygonsCount*3 != polygonIndexes.len() {
            println!("{} {} {}",convertAsList.len(), polygonsCount, polygonIndexes.len());
            return Err( String::from("convertAsList.len()*polygonsCount*3 != polygonIndexes.len()") );
        }


        let mut geometryData=Vec::with_capacity(convertAsList.len()*polygonsCount*3*4);

        let mut piIter=polygonIndexes.iter();

        for i in 0..polygonsCount*3{
            for convertAs in convertAsList.iter() {
                match *convertAs {
                    Some( (sourceData, ref convertTo) ) => {
                        let index=match piIter.next(){
                            Some( i ) => i.clone(),
                            None => return Err(String::from("Unexpected end of polygon indexes")),
                        };

                        match *sourceData{
                            SourceData::Float( _ , ref data) => {
                                match *convertTo {
                                    ConvertTo::to_f32 =>
                                        geometryData.write_f32::<LittleEndian>(data[index]).unwrap(),
                                    ConvertTo::to_i32 =>
                                        geometryData.write_i32::<LittleEndian>(data[index] as i32).unwrap(),
                                }
                            },
                            SourceData::Int( _ , ref data) => {
                                match *convertTo {
                                    ConvertTo::to_f32 =>
                                        geometryData.write_f32::<LittleEndian>(data[index] as f32).unwrap(),
                                    ConvertTo::to_i32 =>
                                        geometryData.write_i32::<LittleEndian>(data[index]).unwrap(),
                                }
                            },
                        }
                    },
                    None => {
                        if piIter.next().is_none() {
                            return Err(String::from("Unexpected end of polygon indexes"));
                        }
                    },
                }
            }
        }

        Ok(
            Geometry{
                material:None,
                data:geometryData,
                polygonsCount:polygonsCount,
            }
        )
    }
}

/*
impl<V:ColladaVertex> Lod<V>{
    pub fn construct(sources:Vec<&Source>, polygonIndexes:Vec<usize>, polygonsCount:usize, geometryType:GeometryType) -> Result<Lod<V>, String>{
        if polygonsCount*V::indexesPerVertex()*geometryType.vertices()!=polygonIndexes.len(){
            return Err( String::from("polygonsCount*indexesPerPolygon!=polygonIndexes.len") );
        }

        let mut vertices=Vec::with_capacity(polygonsCount*geometryType.vertices() );

        let mut index=0;
        loop{
            if index+V::indexesPerVertex() > polygonIndexes.len() {
                break;
            }

            vertices.push( V::construct(&sources,&polygonIndexes[index..]) );

            index+=V::indexesPerVertex();
        }

        Ok(Lod{
            distance:0.0,
            vertices:vertices,
        })
    }
}

impl ColladaVertex for VertexP3N3{
    fn indexesPerVertex() -> usize {
        2
    }

    fn construct(sources:&Vec<&Source>, polygonIndexes:&[usize]) -> VertexP3N3{
        let positions=match *sources[0]{
            Source::XYZ( _, ref data) => data,
            _ => panic!("not XYZ"),
        };

        let normals=match *sources[1]{
            Source::XYZ( _, ref data) => data,
            _ => panic!("not XYZ"),
        };

        let pi=polygonIndexes[0]*3;
        let ni=polygonIndexes[1]*3;

        VertexP3N3{
            p:[ positions[pi+0], positions[pi+2], positions[pi+1], ],
            n:[ normals[ni+0], normals[ni+2], normals[ni+1], ],
        }
    }
}

impl ColladaVertex for VertexP3N3T0C2{
    fn indexesPerVertex() -> usize {
        3
    }

    fn construct(sources:&Vec<&Source>, polygonIndexes:&[usize]) -> VertexP3N3T0C2{
        let positions=match *sources[0]{
            Source::XYZ( _, ref data) => data,
            _ => panic!("not XYZ"),
        };

        let normals=match *sources[1]{
            Source::XYZ( _, ref data) => data,
            _ => panic!("not XYZ"),
        };

        let texcoords=match *sources[2]{
            Source::UV( _, _ , ref data) => data,
            _ => panic!("not UV"),
        };

        let pi=polygonIndexes[0]*3;
        let ni=polygonIndexes[1]*3;
        let tci=polygonIndexes[2]*2;

        VertexP3N3T0C2{
            p:[ positions[pi+0], positions[pi+2], -positions[pi+1], ],
            n:[ normals[ni+0], normals[ni+2], -normals[ni+1], ],
            tc:[ texcoords[tci+0], texcoords[tci+1], ],
        }
    }
}
*/
