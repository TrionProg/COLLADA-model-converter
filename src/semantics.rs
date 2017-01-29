use Error;
use lexer::{Cursor,Lexeme};

pub struct Semantics<'a>{
    pub sources:Vec<SemanticsSource<'a>>,
}

pub struct SemanticsSource<'a>{
    pub name:&'a str,
    pub is_index:bool,
    pub layers:Vec<SemanticsSourceLayer<'a>>,
}

pub struct SemanticsSourceLayer<'a>{
    pub name:&'a str,
    pub layer_type:SemanticsSourceLayerType,
}

pub enum SemanticsSourceLayerType{
    Float,
    Int,
}

impl<'a> Semantics<'a>{
    pub fn parse(semantics_text:&'a String) -> Result<Semantics<'a>, Error>{
        let mut cursor=Cursor::new( semantics_text );

        let mut sources=Vec::new();

        loop{
            match cursor.next()?{
                Lexeme::EOF => break,
                Lexeme::String(source_name) => {
                    let is_index=match cursor.next()?{
                        Lexeme::Ampersand => { cursor.next()?; true},
                        _ => false,
                    };

                    if cursor.lex!=Lexeme::Bracket('(') {
                        return Err( Error::SemanticsParse(format!("Expected (, but {} has been found", cursor.lex.print())) );
                    }

                    let mut layers=Vec::new();

                    loop{
                        match cursor.next()?{
                            Lexeme::String(layer_name) => {
                                if cursor.next()?!=Lexeme::Colon {
                                    return Err( Error::SemanticsParse(format!("Expected :<type> after {} in full vertex semantics, bot {} has been found", layer_name,cursor.lex.print())) );
                                }

                                let layer_type=match cursor.next()?{
                                    Lexeme::String(type_str) => {
                                        match type_str {
                                            "float" => SemanticsSourceLayerType::Float,
                                            "integer" => SemanticsSourceLayerType::Int,
                                            _ => return Err( Error::SemanticsParse(format!("Expected float or integer, but {} has been found", cursor.lex.print())) ),
                                        }
                                    },
                                    _ => return Err( Error::SemanticsParse(format!("Expected float or integer, but {} has been found", cursor.lex.print() )) ),
                                };

                                layers.push(
                                    SemanticsSourceLayer{
                                        name:layer_name,
                                        layer_type:layer_type,
                                    }
                                );

                                match cursor.next()?{
                                    Lexeme::Comma => {},
                                    Lexeme::Bracket(')') => break,
                                    _ => return Err( Error::SemanticsParse(format!("Expected ',' or ')', but {} has been found", cursor.lex.print() )) ),
                                }
                            },
                            _ => return Err( Error::SemanticsParse(format!("Expected name of layer, but {} has been found", cursor.lex.print() )) ),
                        }
                    }

                    sources.push(
                        SemanticsSource{
                            name:source_name,
                            is_index:is_index,
                            layers:layers,
                        }
                    );
                },
                _ => return Err( Error::SemanticsParse(format!("Expected name of source, but {} has been found", cursor.lex.print() )) ),
            }
        }

        Ok(
            Semantics{
                sources:sources,
            }
        )
    }
}
