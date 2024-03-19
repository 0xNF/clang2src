use std::{fmt, iter::Peekable, slice::Iter};

use fancy_regex::Regex;
use lang_c::ast::Identifier;
use serde::Serialize;

use crate::meta::MetaValue;

const KEYWORD_STRUCT: &str = "struct";
const KEYWORD_TYPEDEF: &str = "typedef";
const KEYWORD_CONST: &str = "const";
const KEYWORD_ENUM: &str = "enum";
const KEYWORD_CHAR: &str = "char";
const KEYWORD_VOID: &str = "void";
const KEYWORD_PREPOCESSOR_INCLUDE: &str = "include";
const KEYWORD_PREPOCESSOR_DEFINE: &str = "define";
const KEYWORD_PREPOCESSOR_UNDEF: &str = "undef";
const KEYWORD_PREPROCESSOR_DEFINED: &str = "defined";
const KEYWORD_PREPOCESSOR_IF: &str = "if";
const KEYWORD_PREPOCESSOR_ELSE: &str = "else";
const KEYWORD_PREPOCESSOR_ELIF: &str = "elif";
const KEYWORD_PREPOCESSOR_END_IF: &str = "endif";
const KEYWORD_PREPOCESSOR_ERROR: &str = "error";
const KEYWORD_PREPOCESSOR_IFDEF: &str = "ifdef";
const KEYWORD_PREPOCESSOR_IFNDEF: &str = "ifndef";

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
            ClangTokenType::Hash => parse_preprocessor(&mut iter, &mut ctypes)?,
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
            ClangTokenType::Bang => todo!(),
            ClangTokenType::Minus => todo!(),
            ClangTokenType::Plus => todo!(),
        }
    }
    return Ok(HeaderFile::from(ctypes));
}

fn parse_preprocessor(
    iter: &mut Peekable<Iter<ClangTokenType>>,
    ctypes: &mut Vec<CType>,
) -> Result<(), String> {
    consume_token(iter, ClangTokenType::Hash, false);
    let next = iter.next();
    if let None = next {
        return Err("Invalid final token: #".to_owned());
    }
    match next.unwrap() {
        ClangTokenType::RawIdentifier(val) => {
            if val == KEYWORD_PREPOCESSOR_INCLUDE {
                ctypes.push(parse_preprocessor_include(iter)?);
            } else if val == KEYWORD_PREPOCESSOR_DEFINE {
                ctypes.push(parse_preprocessor_define(iter)?);
            } else if val == KEYWORD_PREPOCESSOR_IF {
                consume_whitespace(iter);
                match iter.next() {
                    Some(ClangTokenType::RawIdentifier(identifier)) => {
                        if identifier == KEYWORD_PREPROCESSOR_DEFINED {
                            consume_until(iter, ClangTokenType::LParen);
                            if let Some(ClangTokenType::RawIdentifier(raw)) = iter.next() {
                                /* check if this value is defined in the previous set of items, and also check that its not 0 (== true) */
                                if !check_ifdef(&ctypes, raw) {
                                    /* skip until the closing #ifdef */
                                    consume_until(iter, ClangTokenType::Hash);
                                    consume_token(
                                        iter,
                                        ClangTokenType::RawIdentifier(
                                            KEYWORD_PREPOCESSOR_END_IF.into(),
                                        ),
                                        true,
                                    );
                                } else {
                                    /* consume whitespace + next rparen and then continue */
                                    consume_whitespace(iter);
                                    consume_token(iter, ClangTokenType::RParen, false);
                                }
                            }
                        } else if identifier == KEYWORD_PREPOCESSOR_IFNDEF {
                            consume_until(iter, ClangTokenType::LParen);
                            if let Some(ClangTokenType::RawIdentifier(raw)) = iter.next() {
                                if !check_ifndef(&ctypes, raw) {
                                    /* skip until the closing #ifdef */
                                    consume_until(iter, ClangTokenType::Hash);
                                }
                            }
                        }
                    }
                    Some(ClangTokenType::Bang) => {
                        todo!("#if !defined() is NYI for clang2src");
                    }
                    Some(_) => {}
                    None => {
                        return Err(
                            "Invalid token following #if statement, termination encountered".into(),
                        )
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
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
                if let CType::Void = param.variable_type.kind {
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
                if (std::mem::discriminant(&ClangTokenType::Comma) == std::mem::discriminant(peek))
                    || (std::mem::discriminant(&ClangTokenType::RParen)
                        == std::mem::discriminant(peek))
                {
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

fn parse_preprocessor_define(iter: &mut Peekable<Iter<ClangTokenType>>) -> Result<CType, String> {
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

fn parse_preprocessor_if(iter: &mut Peekable<Iter<ClangTokenType>>) -> Result<CType, String> {
    Err("Invalid preprocessor if".into())
}

fn parse_preprocessor_include(iter: &mut Peekable<Iter<ClangTokenType>>) -> Result<CType, String> {
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
        if std::mem::discriminant(token) == std::mem::discriminant(&_until) {
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
    inner_required: bool,
) -> &'a ClangTokenType {
    if let Some(t) = iter.next() {
        if std::mem::discriminant(t) == std::mem::discriminant(&token) {
            if inner_required {
                match t {
                    ClangTokenType::Unknown(val) => {
                        if let ClangTokenType::Unknown(inner) = token {
                            if &inner == val {
                                return t;
                            }
                        }
                    }
                    ClangTokenType::Comment(val) => {
                        if let ClangTokenType::Comment(inner) = token {
                            if &inner == val {
                                return t;
                            }
                        }
                    }
                    ClangTokenType::RawIdentifier(val) => {
                        if let ClangTokenType::RawIdentifier(inner) = token {
                            if &inner == val {
                                return t;
                            }
                        }
                    }
                    ClangTokenType::NumericConstant(val) => {
                        if let ClangTokenType::NumericConstant(inner) = token {
                            if &inner == val {
                                return t;
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                return t;
            }
        }
    }
    panic!("Expected a type of token but got something else");
}

/// Given an identifier, checks the CTypes list to see if a `#define $identifier` exists, and is != 0 (i.e., is true).
fn check_ifdef(ctypes: &Vec<CType>, identifier: &str) -> bool {
    !ctypes.iter().all(|x| match x {
        CType::Define(found_id, found_type) => {
            if found_id == identifier {
                match *(found_type.clone()) {
                    CType::Char(v) => {
                        if let Some(l) = v.as_bytes().first() {
                            l != &0
                        } else {
                            false
                        }
                    }
                    CType::SignedShort(v) => v == 0,
                    CType::UnsignedShort(v) => v == 0,
                    CType::SignedInteger(v) => v == 0,
                    CType::UnsignedInteger(v) => v == 0,
                    CType::SignedLong(v) => v == 0,
                    CType::UnsignedLong(v) => v == 0,
                    CType::Int64T(v) => v == 0,
                    CType::Float(v) => v == 0.0,
                    CType::Double(v) => v == 0.0,
                    CType::DoubleDouble(v) => v == 0.0,
                    CType::IntPtrT(v) => v == 0,
                    CType::UIntPtrT(v) => v == 0,
                    CType::Int8T(v) => v == 0,
                    CType::Int16T(v) => v == 0,
                    CType::Int32T(v) => v == 0,
                    CType::UInt8T(v) => v == 0,
                    CType::UInt16T(v) => v == 0,
                    CType::UInt32T(v) => v == 0,
                    CType::UInt64T(v) => v == 0,
                    _ => true,
                }
            } else {
                true
            }
        }
        _ => true,
    })
}

/// Given an identifier, checks the CTypes list to see if a `#define $identifier` doesn't exist, or if it does, if it is == 0 (i.e., is false).
fn check_ifndef(ctypes: &Vec<CType>, identifier: &str) -> bool {
    !check_ifdef(ctypes, identifier)
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
    /// !
    Bang,
}
impl fmt::Display for ClangTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            ClangTokenType::Unknown(val) => format!("{}", val),
            ClangTokenType::Comment(val) => format!("{}", val),
            ClangTokenType::RawIdentifier(val) => format!("{}", val),
            ClangTokenType::NumericConstant(val) => format!("{}", val),
            ClangTokenType::Comma => String::from(","),
            ClangTokenType::RBrace => String::from("{"),
            ClangTokenType::LBrace => String::from("}"),
            ClangTokenType::RParen => String::from("("),
            ClangTokenType::LParen => String::from(")"),
            ClangTokenType::LSquare => String::from("["),
            ClangTokenType::RSquare => String::from("]"),
            ClangTokenType::Semi => String::from(";"),
            ClangTokenType::Period => String::from("."),
            ClangTokenType::Star => String::from("*"),
            ClangTokenType::Greater => String::from(">"),
            ClangTokenType::Less => String::from("<"),
            ClangTokenType::Hash => String::from("#"),
            ClangTokenType::Minus => String::from("-"),
            ClangTokenType::Plus => String::from("+"),
            ClangTokenType::Bang => String::from("!"),
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
            CType::Include(v) => format!("include {}", v).into(),
            CType::Define(label, value) => format!("{} = {}", label, value),
            CType::SignedShort(v) => format!("signed short {}", v).into(),
            CType::UnsignedShort(v) => format!("unsigned short {}", v).into(),
            CType::SignedInteger(v) => format!("signed int {}", v).into(),
            CType::UnsignedInteger(v) => format!("unsigned int {}", v).into(),
            CType::SignedLong(v) => format!("signed long {}", v).into(),
            CType::UnsignedLong(v) => format!("unsigned long {}", v).into(),
            CType::Int64T(v) => format!("int64_t {}", v).into(),
            CType::Float(v) => format!("float {}", v).into(),
            CType::Double(v) => format!("double {}", v).into(),
            CType::DoubleDouble(v) => format!("double double {}", v).into(),
            CType::Char(v) => format!("char {}", v).into(),
            CType::Struct(v) => format!("{}", v).into(),
            CType::Function(v) => format!("{}", v).into(),
            CType::IntPtrT(_) => String::from("intptr_t"),
            CType::UIntPtrT(_) => String::from("uintptr_t"),
            CType::Int8T(_) => String::from("byte_t"),
            CType::Int16T(_) => String::from("int16_t"),
            CType::Int32T(_) => String::from("int32_t"),
            CType::UInt8T(_) => String::from("uint8_t"),
            CType::UInt16T(_) => String::from("uint16_t"),
            CType::UInt32T(_) => String::from("uint32_t"),
            CType::UInt64T(_) => String::from("uint64_t"),
            CType::VoidStar => String::from("void *"),
            CType::Void => String::from("void"),
            CType::UNINITIALIZED => String::from("ERROR VALUE"),
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
