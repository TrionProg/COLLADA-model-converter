use xml::reader::{EventReader, XmlEvent};
use collada::{Parser,skip,waitStartTag,waitEndTag};

use std::collections::BTreeMap;
use std::collections::btree_map::Entry as BTreeMapEntry;

use source::Source;
use mesh::{GeometryType, Mesh, Lod, VertexP3, VertexP3N3, VertexP3N3T0C2};

pub enum Geometry{
    P3N3{
        material:Option<String>,
        lod:Lod<VertexP3N3>,
    },
    P3N3T0C2{
        material:Option<String>,
        lod:Lod<VertexP3N3T0C2>,
    },
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
                                                    "VERTEX" => match *source{
                                                        Source::XYZ ( ref sourceType, _ ) => {
                                                            if sourceType.as_str()!="positions" {
                                                                return Err( format!("VERTEX Slot must be *-position source, found {}", sourceType) );
                                                            }

                                                            vertexSemantics.push_str("p(xyz) ");
                                                        },
                                                        _ => return Err( String::from("VERTEX Slot must be XYZ *-position source") ),
                                                    },
                                                    "NORMAL" => match *source{
                                                        Source::XYZ ( ref sourceType, _ ) => {
                                                            if sourceType.as_str()!="normals" {
                                                                return Err( format!("NORMAL Slot must be *-normals source, found {}", sourceType) );
                                                            }

                                                            vertexSemantics.push_str("n(xyz) ");
                                                        },
                                                        _ => return Err( String::from("NORMAL Slot must be XYZ *-normals source") ),
                                                    },
                                                    "TEXCOORD" => match *source{
                                                        Source::UV ( ref sourceType, ref ti, _ ) => {
                                                            if sourceType.as_str()!="map" {
                                                                return Err( format!("TEXCOORD Slot must be *-map source, found {}", sourceType) );
                                                            }

                                                            vertexSemantics.push_str(&format!("t{}c(uv) ",ti));
                                                        },
                                                        _ => return Err( String::from("TEXCOORD Slot must be XYZ *-normals source") ),
                                                    },
                                                    _ => return Err( format!("Unknown source semantic \"{}\"", semantic) ),
                                                }

                                                sourcesList.push( source );

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

                            match vertexSemantics{
                                "p(xyz) n(xyz)" => {
                                    let lod:Lod<VertexP3N3>=try!(Lod::construct(sourcesList, polygonIndexes, polygonsCount, GeometryType::Triangles));
                                    geometries.push( Geometry::P3N3{
                                        material:specifiedMaterial,
                                        lod:lod,
                                    });
                                },
                                "p(xyz) n(xyz) t0c(uv)" => {
                                    let lod:Lod<VertexP3N3T0C2>=try!(Lod::construct(sourcesList, polygonIndexes, polygonsCount, GeometryType::Triangles));
                                    println!("{}",lod.vertices.len());
                                    geometries.push( Geometry::P3N3T0C2{
                                        material:specifiedMaterial,
                                        lod:lod,
                                    });
                                },
                                _ => return Err( format!("Unknown vertex semantics: {}", vertexSemantics) ),
                            }
                        },
                        _ => try!( skip( parser, name.local_name) ),
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
}

trait ColladaVertex{
    fn indexesPerVertex() -> usize;
    fn construct(sources:&Vec<&Source>, polygonIndexes:&[usize]) -> Self;
}

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
