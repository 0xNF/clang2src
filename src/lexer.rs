use std::{fmt, iter::Peekable, slice::Iter};

use fancy_regex::Regex;
use serde::Serialize;

use crate::meta::MetaValue;

const KEYWORD_STRUCT: &str = "struct";
const KEYWORD_TYPEDEF: &str = "typedef";
const KEYWORD_CONST: &str = "const";
const KEYWORD_INCLUDE: &str = "include";
const KEYWORD_DEFINE: &str = "define";
const KEYWORD_ENUM: &str = "enum";
const KEYWORD_CHAR: &str = "char";
const KEYWORD_VOID: &str = "void";

pub struct HeaderFile {
    pub includes: Vec<String>,
    pub defines: Vec<CVariableDeclaration>,
    pub enums: Vec<CEnum>,
    pub structs: Vec<CStruct>,
    pub functions: Vec<CFunction>,
}

impl From<Vec<CType>> for HeaderFile {
    fn from(lst: Vec<CType>) -> Self {
        let mut hf = HeaderFile {
            includes: vec![],
            defines: vec![],
            enums: vec![],
            structs: vec![],
            functions: vec![],
        };

        for ctype in lst.into_iter() {
            match ctype {
                CType::Include(s) => hf.includes.push(s.to_owned()),
                CType::Define(label, decl) => hf.defines.push(CVariableDeclaration {
                    label: label.to_owned(),
                    comment: None,
                    meta: None,
                    is_const: true,
                    variable_type: CVariableType {
                        kind: *decl,
                        is_struct: false,
                        pointer_count: 0,
                    },
                }),
                CType::Enum(e) => hf.enums.push(e),
                CType::Struct(s) => hf.structs.push(s),
                CType::Function(f) => hf.functions.push(f),
                _ => continue,
            }
        }

        hf
    }
}

pub fn tokenize(token_str: &str) -> Vec<ClangTokenType> {
    let mut tokens: Vec<ClangTokenType> = vec![];
    let r = Regex::new(r"([a-z_]+) '((//[^\n]*$|/(?!\\)\*[\s\S]*?\*(?!\\)/)|.*[\s]*)'").unwrap();
    let captures = r.captures_iter(token_str);
    for capture in captures.into_iter() {
        match capture {
            Err(_) => continue,
            Ok(c) => {
                let kind = c.get(1).unwrap().as_str();
                let val = c.get(2).unwrap().as_str().to_owned();
                match kind {
                    "minus" => tokens.push(ClangTokenType::Minus),
                    "plus" => tokens.push(ClangTokenType::Plus),
                    "hash" => tokens.push(ClangTokenType::Hash),
                    "less" => tokens.push(ClangTokenType::Less),
                    "greater" => tokens.push(ClangTokenType::Greater),
                    "star" => tokens.push(ClangTokenType::Star),
                    "period" => tokens.push(ClangTokenType::Period),
                    "semi" => tokens.push(ClangTokenType::Semi),
                    "l_paren" => tokens.push(ClangTokenType::LParen),
                    "r_paren" => tokens.push(ClangTokenType::RParen),
                    "l_brace" => tokens.push(ClangTokenType::LBrace),
                    "r_brace" => tokens.push(ClangTokenType::RBrace),
                    "l_square" => tokens.push(ClangTokenType::LSquare),
                    "r_square" => tokens.push(ClangTokenType::RSquare),
                    "comma" => tokens.push(ClangTokenType::Comma),
                    "comment" => tokens.push(ClangTokenType::Comment(val)),
                    "numeric_constant" => {
                        let num = val.parse::<f64>().unwrap();
                        tokens.push(ClangTokenType::NumericConstant(num));
                    }
                    "raw_identifier" => tokens.push(ClangTokenType::RawIdentifier(val)),
                    _ => tokens.push(ClangTokenType::Unknown(val)),
                }
            }
        }
    }
    return tokens;
}

pub fn parse(tokens: Vec<ClangTokenType>) -> Result<HeaderFile, String> {
    let mut ctypes: Vec<CType> = vec![];

    let mut iter = &mut tokens.iter().peekable();
    while let Some(token) = iter.peek() {
        match token {
            ClangTokenType::Unknown(_) => {
                iter.next();
            }
            ClangTokenType::Comment(cmt) => {
                if let ClangTokenType::Comment(_) = consume_whitespace(iter).unwrap() {}
                consume_token(iter, ClangTokenType::Comment("".to_owned()), false);
                consume_whitespace(iter);
                let peek = iter.peek().unwrap();
                match peek {
                    ClangTokenType::RawIdentifier(val) => {
                        if val == KEYWORD_TYPEDEF {
                            consume_token(
                                iter,
                                ClangTokenType::RawIdentifier(KEYWORD_TYPEDEF.to_owned()),
                                false,
                            );
                            iter.next();
                            ctypes.push(parse_type(iter, Some(cmt.to_owned()))?);
                        } else {
                            ctypes.push(parse_function(iter, Some(cmt.to_owned()))?);
                        }
                    }
                    ClangTokenType::Hash => continue,
                    _ => {
                        iter.next();
                    }
                }
            }
            ClangTokenType::RawIdentifier(val) => {
                if val == KEYWORD_TYPEDEF {
                    consume_token(
                        iter,
                        ClangTokenType::RawIdentifier(KEYWORD_TYPEDEF.to_owned()),
                        false,
                    );
                    consume_whitespace(iter);
                    ctypes.push(parse_type(iter, None)?);
                } else {
                    ctypes.push(parse_function(iter, None)?);
                    // println!("{}", ctypes[ctypes.len() - 1]);
                }
            }
            ClangTokenType::NumericConstant(_) => todo!(),
            ClangTokenType::Comma => todo!(),
            ClangTokenType::RBrace => todo!(),
            ClangTokenType::LBrace => todo!(),
            ClangTokenType::RParen => todo!(),
            ClangTokenType::LParen => todo!(),
            ClangTokenType::LSquare => todo!(),
            ClangTokenType::RSquare => todo!(),
            ClangTokenType::Semi => todo!(),
            ClangTokenType::Period => todo!(),
            ClangTokenType::Star => todo!(),
            ClangTokenType::Greater => todo!(),
            ClangTokenType::Less => todo!(),
            ClangTokenType::Hash => {
                consume_token(iter, ClangTokenType::Hash, false);
                let next = iter.next();
                if let None = next {
                    return Err("Invalid final token: #".to_owned());
                }
                match next.unwrap() {
                    ClangTokenType::RawIdentifier(val) => {
                        if val == KEYWORD_INCLUDE {
                            ctypes.push(parse_include(&mut iter)?);
                        } else if val == KEYWORD_DEFINE {
                            ctypes.push(parse_define(&mut iter)?);
                        }
                    }
                    _ => {}
                }
            }
            ClangTokenType::Minus => todo!(),
            ClangTokenType::Plus => todo!(),
        }
    }
    return Ok(HeaderFile::from(ctypes));
}

fn parse_type(
    iter: &mut Peekable<Iter<ClangTokenType>>,
    comment: Option<String>,
) -> Result<CType, String> {
    let peek = iter.peek();
    if let None = peek {
        return Err("Invalid token: nothing after typedef".to_owned());
    }
    let kind = peek.unwrap();
    match kind {
        ClangTokenType::RawIdentifier(val) => {
            if val == KEYWORD_ENUM {
                parse_enum(iter, comment)
            } else if val == KEYWORD_STRUCT {
                parse_struct(iter, comment)
            } else {
                parse_function(iter, comment)
            }
        }
        _ => Err("Invalid token: nothing after typedef".to_owned()),
    }
}

fn parse_enum(
    iter: &mut Peekable<Iter<ClangTokenType>>,
    comment: Option<String>,
) -> Result<CType, String> {
    iter.next(); // Consume `enum` token
    consume_whitespace(iter);
    let label: String;
    let mut declarations: Vec<CIdentifier> = vec![];
    if let ClangTokenType::RawIdentifier(val) = iter.next().unwrap() {
        label = val.to_owned();
    } else {
        return Err("Invalid enum: No identifier".to_owned());
    };
    consume_until(iter, ClangTokenType::LBrace);
    consume_whitespace(iter);

    let mut current_comment: Option<String> = None;

    while let Some(token) = iter.next() {
        match token {
            ClangTokenType::RBrace => {
                end_struct_with_name(iter, &label)?;
                return Ok(CType::Enum(CEnum {
                    identifier: CIdentifier {
                        label: label.to_owned(),
                        comment: None,
                    },
                    meta: MetaValue::from_meta_comment_dontcare(&comment),
                    comment,
                    declarations,
                }));
            }
            ClangTokenType::Comma => continue,
            ClangTokenType::Comment(cmt) => current_comment = Some(cmt.to_owned()),
            ClangTokenType::RawIdentifier(identifier) => {
                declarations.push(CIdentifier {
                    label: identifier.to_owned(),
                    comment: current_comment,
                });
                current_comment = None;
            }
            _ => continue,
        }
    }
    Err("Invalid Enum: Failed to parse tokens".to_owned())
}

fn parse_struct(
    iter: &mut Peekable<Iter<ClangTokenType>>,
    comment: Option<String>,
) -> Result<CType, String> {
    iter.next(); // Consume `struct` token
    consume_whitespace(iter);
    let label: String;
    let mut members: Vec<CVariableDeclaration> = vec![];
    if let ClangTokenType::RawIdentifier(val) = iter.next().unwrap() {
        label = val.to_owned();
    } else {
        return Err("Invalid struct: No identifier".to_owned());
    };
    consume_whitespace(iter);
    /* handle empty structs */
    let next = iter.peek().unwrap();
    if let ClangTokenType::RawIdentifier(_) = next {
        consume_until(iter, ClangTokenType::Semi);
        iter.next(); // consume the semi
        return Ok(CType::Struct(CStruct {
            identifier: CIdentifier {
                label: label.to_owned(),
                comment: None,
            },
            meta: MetaValue::from_meta_comment_dontcare(&comment),
            comment,
            declarations: members,
        }));
    }

    consume_until(iter, ClangTokenType::LBrace);
    iter.next(); // consume lbrace
    consume_whitespace(iter);

    let mut current_comment: Option<String> = None;
    while let Some(token) = iter.peek() {
        match token {
            ClangTokenType::RBrace => {
                iter.next(); // consume rbrace
                end_struct_with_name(iter, &label)?;
                return Ok(CType::Struct(CStruct {
                    identifier: CIdentifier {
                        label: label.to_owned(),
                        comment: None,
                    },
                    meta: MetaValue::from_meta_comment_dontcare(&comment),

                    comment,
                    declarations: members,
                }));
            }
            ClangTokenType::Comment(cmt) => {
                current_comment = Some(cmt.to_owned());
                iter.next();
            }
            _ => {
                members.push(parse_struct_member(iter, current_comment)?);
                current_comment = None;
            }
        }
        consume_whitespace(iter);
    }

    let s = CStruct {
        identifier: CIdentifier {
            label: label.to_owned(),
            comment: None,
        },
        meta: MetaValue::from_meta_comment_dontcare(&comment),
        comment,
        declarations: members,
    };
    Ok(CType::Struct(s))
}

fn parse_struct_member(
    iter: &mut Peekable<Iter<ClangTokenType>>,
    comment: Option<String>,
) -> Result<CVariableDeclaration, String> {
    consume_whitespace(iter);
    let mut label: &str = "";
    let mut pointer_count = 0;
    let mut is_const: bool = false;
    let mut is_struct: bool = false;
    let mut is_enum: bool = false;

    let mut signature: Vec<&str> = vec![];
    while let Some(token) = iter.next() {
        match token {
            ClangTokenType::RawIdentifier(val) => {
                if let ClangTokenType::Semi = iter.peek().unwrap() {
                    /* this is the label  */
                    label = val;
                } else if val == KEYWORD_CONST {
                    is_const = true;
                } else if val == KEYWORD_STRUCT {
                    is_struct = true;
                } else if val == KEYWORD_ENUM {
                    is_enum = true;
                } else {
                    signature.push(val);
                }
            }
            ClangTokenType::Star => pointer_count += 1,
            ClangTokenType::Semi => {
                let variable_type =
                    match_variable_signature(signature, is_struct, is_enum, pointer_count)?;
                let variable_decl = CVariableDeclaration {
                    meta: MetaValue::from_meta_comment_dontcare(&comment),
                    comment: match &comment {
                        Some(cmt) => Some(cmt.to_owned()),
                        None => None,
                    },
                    is_const,
                    label: label.to_owned(),
                    variable_type,
                };

                return Ok(variable_decl);
            }
            _ => return Err("Invalid struct member".to_owned()),
        }
        consume_whitespace(iter);
    }
    Err("Struct Member: Failed to parse variable member".to_owned())
}

fn match_variable_signature(
    signature: Vec<&str>,
    is_struct: bool,
    is_enum: bool,
    pointer_count: u8,
) -> Result<CVariableType, String> {
    Ok(CVariableType {
        kind: match signature.join(" ").as_str() {
            KEYWORD_CHAR | "signed char" | "unsigned char" => CType::Char("".to_owned()),
            "short" | "short int" | "signed short" | "signed short int" => CType::SignedShort(0),
            "unsigned short" | "unsigned short int" => CType::UnsignedShort(0),
            "int" | "signed" | "signed int" => CType::SignedShort(0),
            "unsigned" | "unsigned int" => CType::UnsignedShort(0),
            "long" | "long int" | "signed long" | "signed long int" => CType::SignedInteger(0),
            "unsigned long" | "unsigned long int" => CType::UnsignedInteger(0),
            "long long" | "long long int" | "signed long long" | "signed long long int" => {
                CType::SignedLong(0)
            }
            "unsigned long long" | "unsigned long long int" => CType::UnsignedLong(0),
            "float" => CType::Float(0.0),
            "double" => CType::Double(0.0),
            "long double" => CType::DoubleDouble(0.0),
            "int8_t" => CType::Int8T(0),
            "int16_t" => CType::Int16T(0),
            "int32_t" => CType::Int32T(0),
            "int64_t" => CType::Int64T(0),
            "uint8_t" => CType::UInt8T(0),
            "uint16_t" => CType::UInt16T(0),
            "uint32_t" => CType::UInt32T(0),
            "uint64_t" => CType::UInt64T(0),
            "intptr_t" => CType::IntPtrT(0),
            "uintptr_t" => CType::UIntPtrT(0),
            "void *" => CType::VoidStar,
            KEYWORD_VOID => CType::Void,
            _ => {
                if is_struct {
                    // FYI(nf): dummy struct
                    CType::Struct(CStruct {
                        identifier: CIdentifier {
                            label: signature.first().unwrap().to_string(),
                            comment: None,
                        },
                        meta: None,
                        comment: None,
                        declarations: vec![],
                    })
                } else if is_enum {
                    // FYI(nf): dummy enum
                    CType::Enum(CEnum {
                        identifier: CIdentifier {
                            label: signature.first().unwrap().to_string(),
                            comment: None,
                        },
                        meta: None,
                        comment: None,
                        declarations: vec![],
                    })
                } else {
                    return Err("Invalid struct member: not a valid c-type".to_owned());
                }
            }
        },
        is_struct,
        pointer_count,
    })
}

fn end_struct_with_name(
    iter: &mut Peekable<Iter<ClangTokenType>>,
    struct_name: &str,
) -> Result<(), String> {
    consume_whitespace(iter);
    /* consume the trailing strutc type name as well */
    if let Some(t) = iter.peek() {
        if let ClangTokenType::RawIdentifier(val2) = t {
            if val2.to_owned() == struct_name.to_owned() {
                iter.next();
            } else {
                return Err("Invalid struct: Failed to end struct with struct name".to_owned());
            }
        } else {
            return Err("Invalid struct: Failed to end struct with struct name".to_owned());
        }
    } else {
        return Err("Invalid struct: Failed to end struct with struct name".to_owned());
    }
    consume_until(iter, ClangTokenType::Semi);
    Ok(())
}

fn parse_function(
    iter: &mut Peekable<Iter<ClangTokenType>>,
    comment: Option<String>,
) -> Result<CType, String> {
    let mut label: &str = "";

    /* Get the Return Value */
    let mut return_signature: Vec<&str> = vec![];
    let mut return_pointer_count: u8 = 0;
    let mut return_is_struct: bool = false;
    let mut return_is_enum: bool = false;

    while let Some(token) = iter.peek() {
        match token {
            /* the immediate prior token to a '(' is the function name */
            ClangTokenType::LParen => {
                let lbl = return_signature.remove(return_signature.len() - 1);
                label = lbl;
                break;
            }
            /* set pointers */
            ClangTokenType::Star => return_pointer_count += 1,
            ClangTokenType::RawIdentifier(val) => {
                /* set is Struct */
                if val == KEYWORD_STRUCT {
                    return_is_struct = true;
                } else if val == KEYWORD_ENUM {
                    return_is_enum = true;
                } else {
                    return_signature.push(val);
                }
            }
            /* ignore whitespace */
            ClangTokenType::Unknown(_) => (),
            _ => (),
        };
        iter.next();
        consume_whitespace(iter);
    }

    let return_type = match_variable_signature(
        return_signature,
        return_is_struct,
        return_is_enum,
        return_pointer_count,
    )?;

    iter.next(); /* consume l-paren */

    let mut parameters: Vec<CVariableDeclaration> = vec![];

    let mut current_comment: Option<String> = None;
    while let Some(token) = iter.peek() {
        match token {
            ClangTokenType::RParen => {
                consume_until(iter, ClangTokenType::Semi);
                iter.next(); /* consume the semi as well */
                return Ok(CType::Function(CFunction {
                    return_type: Box::new(return_type),
                    label: label.to_string(),
                    meta: MetaValue::from_meta_comment_dontcare(&comment),
                    comment,
                    parameters,
                }));
            }
            ClangTokenType::Comment(cmt) => {
                current_comment = Some(cmt.to_owned());
                iter.next();
            }
            ClangTokenType::Semi => {
                iter.next(); /* consume the semi */
                break;
            }
            _ => {
                let param = parse_function_parameter(iter, &comment)?;
                if matches!(param.variable_type.kind, CType::Void) {
                    parameters.clear();
                } else {
                    parameters.push(param);
                }
                current_comment = None;
            }
        }
        consume_whitespace(iter);
    }
    /* Get the Parametes */

    let func: CFunction = CFunction {
        return_type: Box::new(return_type),
        label: label.to_string(),
        meta: MetaValue::from_meta_comment_dontcare(&comment),
        comment,
        parameters,
    };

    Ok(CType::Function(func))
}

fn parse_function_parameter(
    iter: &mut Peekable<Iter<ClangTokenType>>,
    comment: &Option<String>,
) -> Result<CVariableDeclaration, String> {
    consume_whitespace(iter);
    let mut label: &str = "";
    let mut pointer_count = 0;
    let mut is_const: bool = false;
    let mut is_struct: bool = false;
    let mut is_enum: bool = false;

    let mut signature: Vec<&str> = vec![];
    while let Some(token) = iter.next() {
        match token {
            ClangTokenType::RawIdentifier(val) => {
                if val == KEYWORD_VOID {
                    /* skip parsing parameters, this is a void function */
                    consume_until(iter, ClangTokenType::RParen);
                    /* fyi(nf): dummy variable for void parameter functions */
                    return Ok(CVariableDeclaration {
                        comment: None,
                        meta: None,
                        is_const: false,
                        label: "".to_owned(),
                        variable_type: CVariableType {
                            kind: CType::Void,
                            is_struct: false,
                            pointer_count: 0,
                        },
                    });
                }
                let peek = iter.peek().unwrap();
                if matches!(peek, ClangTokenType::Comma | ClangTokenType::RParen) {
                    /* this is the label  */
                    label = val;
                } else if val == KEYWORD_CONST {
                    is_const = true;
                } else if val == KEYWORD_STRUCT {
                    is_struct = true;
                } else if val == KEYWORD_ENUM {
                    is_enum = true;
                } else {
                    signature.push(val);
                }
            }
            ClangTokenType::Star => pointer_count += 1,
            ClangTokenType::Comma | ClangTokenType::RParen => {
                let variable_type =
                    match_variable_signature(signature, is_struct, is_enum, pointer_count)?;
                let variable_decl = CVariableDeclaration {
                    meta: MetaValue::from_meta_comment_for_param(&comment, label),
                    comment: match &comment {
                        Some(cmt) => Some(cmt.to_owned()),
                        None => None,
                    },
                    is_const,
                    label: label.to_owned(),
                    variable_type,
                };

                return Ok(variable_decl);
            }
            _ => return Err("Invalid function parameter member".to_owned()),
        }
        consume_whitespace(iter);
    }
    Err("function parameter: Failed to parse variable member".to_owned())
}

fn parse_define(iter: &mut Peekable<Iter<ClangTokenType>>) -> Result<CType, String> {
    let mut label: &str = "";
    let mut is_negative: bool = false;
    let mut ctype: CType = CType::UNINITIALIZED;
    consume_whitespace(iter);
    while let Some(token) = iter.next() {
        match token {
            ClangTokenType::Unknown(_) => continue,
            ClangTokenType::RawIdentifier(val) => {
                if label == "" {
                    label = val;
                } else {
                    ctype = CType::Char(val.to_owned());
                    break;
                }
            }
            ClangTokenType::Minus => is_negative = true,
            ClangTokenType::NumericConstant(val) => {
                let mut d: f64 = *val;
                if is_negative {
                    d = -d;
                }
                if d.fract() == 0.0 {
                    let i: i32 = d as i32;
                    ctype = CType::SignedInteger(i);
                    break;
                } else {
                    ctype = CType::Double(d);
                    break;
                }
            }

            _ => return Err("Invalid Define".to_owned()),
        }
    }

    return Ok(CType::Define(label.to_owned(), Box::new(ctype)));
}

fn parse_include(iter: &mut Peekable<Iter<ClangTokenType>>) -> Result<CType, String> {
    let mut label: Vec<&str> = vec![];
    let mut is_open: bool = false;
    while let Some(token) = iter.next() {
        match token {
            ClangTokenType::Unknown(_) => continue,
            ClangTokenType::Less => is_open = true,
            ClangTokenType::RawIdentifier(val) => {
                if is_open {
                    label.push(val);
                } else {
                    return Err("Invalid Include: label given but not open".to_owned());
                }
            }
            ClangTokenType::Period => {
                if is_open {
                    label.push(".");
                } else {
                    return Err("Invalid Include: label not open".to_owned());
                }
            }
            ClangTokenType::Greater => {
                if is_open {
                    is_open = false;
                    break;
                } else {
                    return Err("Invalid Include: Given close, but not open".to_owned());
                }
            }
            _ => return Err("Invalid Include: not valid format".to_owned()),
        }
    }
    if label.len() != 0 && !is_open {
        Ok(CType::Include(label.join("")))
    } else {
        Err("Invald Include: No label found".to_owned())
    }
}

fn consume_until(iter: &mut Peekable<Iter<ClangTokenType>>, _until: ClangTokenType) {
    while let Some(token) = iter.next() {
        if matches!(token, _until) {
            return;
        }
    }
}

/// Moves the iterator forward each time the Next Token (iter.peek) is Whitespace
///
/// Returns the first token that isn't whitespace (aka)
fn consume_whitespace<'a>(
    iter: &'a mut Peekable<Iter<ClangTokenType>>,
) -> Option<&'a ClangTokenType> {
    while let Some(peek) = iter.peek() {
        match peek {
            ClangTokenType::Unknown(_) => {
                iter.next();
            }
            _ => return Some(peek),
        }
    }
    None
}

/// Moves the iterator forward by one and eats the specified next token.
///
///  Panics if the token isnt of the correct type.
///
/// Returns the token found [aka, Iter.CurrentPosition]
fn consume_token<'a>(
    iter: &'a mut Peekable<Iter<ClangTokenType>>,
    token: ClangTokenType,
    _inner_required: bool,
) -> &'a ClangTokenType {
    if let Some(t) = iter.next() {
        if std::mem::discriminant(t) == std::mem::discriminant(&token) {
            // if inner_required {
            //     if let ClangTokenType::RawIdentifier(val) = t {
            //         return val == token.
            //     }
            // } else {
            return t;
            // }
        }
    }
    panic!("Expected a type of token but got something else");
}

#[derive(Debug, PartialEq)]
pub enum ClangTokenType {
    /// Usually just whitespace
    Unknown(String),
    /// Either // or /* [...] */
    Comment(String),
    /// Anything else
    RawIdentifier(String),
    /// Numbers (all ints are doubles)
    NumericConstant(f64),
    /// ,
    Comma,
    /// {
    RBrace,
    /// }
    LBrace,
    /// (
    RParen,
    /// )
    LParen,
    /// [
    LSquare,
    /// ]
    RSquare,
    /// ;
    Semi,
    /// .
    Period,
    /// *
    Star,
    /// >
    Greater,
    /// <
    Less,
    /// #
    Hash,
    /// -
    Minus,
    /// +
    Plus,
}
impl fmt::Display for ClangTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            ClangTokenType::Unknown(val) => format!("{}", val),
            ClangTokenType::Comment(val) => format!("{}", val),
            ClangTokenType::RawIdentifier(val) => format!("{}", val),
            ClangTokenType::NumericConstant(val) => format!("{}", val),
            ClangTokenType::Comma => ",".to_owned(),
            ClangTokenType::RBrace => "{".to_owned(),
            ClangTokenType::LBrace => "}".to_owned(),
            ClangTokenType::RParen => "(".to_owned(),
            ClangTokenType::LParen => ")".to_owned(),
            ClangTokenType::LSquare => "[".to_owned(),
            ClangTokenType::RSquare => "]".to_owned(),
            ClangTokenType::Semi => ";".to_owned(),
            ClangTokenType::Period => ".".to_owned(),
            ClangTokenType::Star => "*".to_owned(),
            ClangTokenType::Greater => ">".to_owned(),
            ClangTokenType::Less => "<".to_owned(),
            ClangTokenType::Hash => "#".to_owned(),
            ClangTokenType::Minus => "-".to_owned(),
            ClangTokenType::Plus => "+".to_owned(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum CType {
    Enum(CEnum),
    Include(String),
    /// 16bit
    SignedShort(i16),
    /// 16bit
    UnsignedShort(u16),
    /// 32bit
    SignedInteger(i32),
    /// 32bit
    UnsignedInteger(u32),
    /// 64 bit
    SignedLong(i64),
    /// 64 bit
    UnsignedLong(u64),
    Int64T(i64),
    /// 32bit float (single)
    Float(f32),
    /// 64 bit float (double)
    Double(f64),
    /// FYI(nf): rust doesnt handle this
    ///
    /// 128bit quad precision
    DoubleDouble(f64),
    /// 8 bit or also string
    Char(String),
    Struct(CStruct),
    Function(CFunction),
    /// {Label, Type(value)}
    Define(String, Box<CType>),
    IntPtrT(i32),
    UIntPtrT(u32),
    Int8T(i8),
    Int16T(i16),
    Int32T(i32),
    UInt8T(u8),
    UInt16T(u16),
    UInt32T(u32),
    UInt64T(u64),
    VoidStar,
    Void,
    UNINITIALIZED,
}
impl fmt::Display for CType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            CType::Enum(v) => v.to_string(),
            CType::Include(v) => format!("include {}", v).to_owned(),
            CType::Define(label, value) => format!("{} = {}", label, value),
            CType::SignedShort(v) => format!("signed short {}", v).to_owned(),
            CType::UnsignedShort(v) => format!("unsigned short {}", v).to_owned(),
            CType::SignedInteger(v) => format!("signed int {}", v).to_owned(),
            CType::UnsignedInteger(v) => format!("unsigned int {}", v).to_owned(),
            CType::SignedLong(v) => format!("signed long {}", v).to_owned(),
            CType::UnsignedLong(v) => format!("unsigned long {}", v).to_owned(),
            CType::Int64T(v) => format!("int64_t {}", v).to_owned(),
            CType::Float(v) => format!("float {}", v).to_owned(),
            CType::Double(v) => format!("double {}", v).to_owned(),
            CType::DoubleDouble(v) => format!("double double {}", v).to_owned(),
            CType::Char(v) => format!("char {}", v).to_owned(),
            CType::Struct(v) => format!("{}", v).to_owned(),
            CType::Function(v) => format!("{}", v).to_owned(),
            CType::IntPtrT(_) => "intptr_t".to_owned(),
            CType::UIntPtrT(_) => "uintptr_t".to_owned(),
            CType::Int8T(_) => "byte_t".to_owned(),
            CType::Int16T(_) => "int16_t".to_owned(),
            CType::Int32T(_) => "int32_t".to_owned(),
            CType::UInt8T(_) => "uint8_t".to_owned(),
            CType::UInt16T(_) => "uint16_t".to_owned(),
            CType::UInt32T(_) => "uint32_t".to_owned(),
            CType::UInt64T(_) => "uint64_t".to_owned(),
            CType::VoidStar => "void *".to_owned(),
            CType::Void => "void".to_owned(),
            CType::UNINITIALIZED => "ERROR VALUE".to_owned(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct CEnum {
    pub identifier: CIdentifier,
    pub comment: Option<String>,
    pub meta: Option<MetaValue>,
    pub declarations: Vec<CIdentifier>,
}
impl fmt::Display for CEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "enum {}{{{}}};", self.identifier.label, "TODO_MEMBERS")
    }
}

#[derive(Debug, Clone)]
pub struct CStruct {
    pub identifier: CIdentifier,
    pub comment: Option<String>,
    pub meta: Option<MetaValue>,
    pub declarations: Vec<CVariableDeclaration>,
}
impl fmt::Display for CStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier.label)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CIdentifier {
    pub label: String,
    pub comment: Option<String>,
}
impl CIdentifier {
    pub fn new(label: &str, comment: Option<&str>) -> Self {
        return CIdentifier {
            label: label.to_owned(),
            comment: comment.map(|f| f.to_owned()),
        };
    }
}
impl fmt::Display for CIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "var {}", self.label)
    }
}

#[derive(Debug, Clone)]
pub struct CConstant {
    pub kind: CType,
    pub label: String,
    pub comment: Option<String>,
}
impl fmt::Display for CConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {};", self.kind.to_string(), self.label,)
    }
}

#[derive(Debug, Clone)]
pub struct CFunction {
    pub return_type: Box<CVariableType>,
    pub label: String,
    pub comment: Option<String>,
    pub meta: Option<MetaValue>,
    pub parameters: Vec<CVariableDeclaration>,
}

impl fmt::Display for CFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}({});",
            self.return_type.to_string(),
            self.label,
            "TODO_PARAM",
        )
    }
}

#[derive(Debug, Clone)]
pub struct CVariableDeclaration {
    pub label: String,
    pub comment: Option<String>,
    pub is_const: bool,
    pub variable_type: CVariableType,
    pub meta: Option<MetaValue>,
}

impl fmt::Display for CVariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_const = if self.is_const { "const " } else { "" };
        write!(
            f,
            "{}{}{}",
            str_const,
            self.variable_type.to_string(),
            self.label
        )
    }
}

#[derive(Debug, Clone)]
pub struct CVariableType {
    pub kind: CType,
    pub is_struct: bool,
    pub pointer_count: u8,
}
impl fmt::Display for CVariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
        // let str_struct = if self.is_struct { "struct " } else { "" };
        // let str_ptr = "TODO*";

        // let ptrcnt: usize = 0;
        // while ptrcnt < self.pointer_count {

        // }
        // for let i = 0; i < self.pointer_count; i++ {

        // }
        // let str_ptr = "*".repeat(self.pointer_count.into::<usize>());
        // write!(f, "{}{}{}", str_struct, str_ptr, self.kind.to_string())
    }
}
