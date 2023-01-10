use std::{collections::HashMap, fmt};

use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize, Serializer,
};
use serde_json::Value;
use tera::{Context, Tera};

use crate::{
    lexer::{
        CEnum, CFunction, CIdentifier, CStruct, CType, CVariableDeclaration, CVariableType,
        HeaderFile,
    },
    meta::MetaValue,
};

const FIELD_PTR: &'static str = "ptr";
const FIELD_SELF: &'static str = "_obj";
const C_PREFACE: &'static str = "c_";
const C_STRUCT_PREFACE: &'static str = "C.struct_";

#[derive(Clone)]
enum GoTypeBasic {
    Error,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uintptr,
    Bool,
    String,
    Byte,
    Rune,
    Float32,
    Float64,
}

impl Serialize for GoTypeBasic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl fmt::Display for GoTypeBasic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            GoTypeBasic::Int => "int",
            GoTypeBasic::Int8 => "int8",
            GoTypeBasic::Int16 => "int16",
            GoTypeBasic::Int32 => "int32",
            GoTypeBasic::Int64 => "int64",
            GoTypeBasic::Uint => "uint",
            GoTypeBasic::Uint8 => "uint8",
            GoTypeBasic::Uint16 => "uint16",
            GoTypeBasic::Uint32 => "uint32",
            GoTypeBasic::Uint64 => "uint64",
            GoTypeBasic::Uintptr => "uintptr",
            GoTypeBasic::Bool => "bool",
            GoTypeBasic::String => "string",
            GoTypeBasic::Byte => "byte",
            GoTypeBasic::Rune => "rune",
            GoTypeBasic::Float32 => "float32",
            GoTypeBasic::Float64 => "float64",
            GoTypeBasic::Error => "error",
        };
        f.write_fmt(format_args!("{}", s))
    }
}

impl GoTypeBasic {
    fn make_go_value(&self, val: &str) -> String {
        match &self {
            GoTypeBasic::String => format!("\"{}\"", val),
            GoTypeBasic::Bool => match val.to_lowercase().as_str() {
                "true" => "true",
                "false" => "false",
                _ => panic!("Invalid boolean value in GoTypeBasic. Got {}", val),
            }
            .to_owned(),
            _ => val.to_owned(),
        }
    }

    fn from_c_value(&self, val: &str) -> String {
        match &self {
            GoTypeBasic::Error => "TODO(nf, not_implemented_from_c_value) error".to_owned(),
            GoTypeBasic::Int => "TODO(nf, not_implemented_from_c_value) int".to_owned(),
            GoTypeBasic::Int8 => "TODO(nf, not_implemented_from_c_value) int8".to_owned(),
            GoTypeBasic::Int16 => "TODO(nf, not_implemented_from_c_value) int16".to_owned(),
            GoTypeBasic::Int32 => "TODO(nf, not_implemented_from_c_value) i32".to_owned(),
            GoTypeBasic::Int64 => "TODO(nf, not_implemented_from_c_value) i64".to_owned(),
            GoTypeBasic::Uint => format!("uint({})", val).to_owned(),
            GoTypeBasic::Uint8 => "TODO(nf, not_implemented_from_c_value) u8".to_owned(),
            GoTypeBasic::Uint16 => "TODO(nf, not_implemented_from_c_value) u16".to_owned(),
            GoTypeBasic::Uint32 => "TODO(nf, not_implemented_from_c_value) u32".to_owned(),
            GoTypeBasic::Uint64 => "TODO(nf, not_implemented_from_c_value) u64".to_owned(),
            GoTypeBasic::Uintptr => format!("uint({})", val).to_owned(),
            GoTypeBasic::Bool => "TODO(nf, not_implemented_from_c_value) bool".to_owned(),
            GoTypeBasic::String => format!("C.GoString({})", val).to_owned(),
            GoTypeBasic::Byte => "TODO(nf, not_implemented_from_c_value) byte".to_owned(),
            GoTypeBasic::Rune => "TODO(nf, not_implemented_from_c_value) rune".to_owned(),
            GoTypeBasic::Float32 => "TODO(nf, not_implemented_from_c_value) f32".to_owned(),
            GoTypeBasic::Float64 => "TODO(nf, not_implemented_from_c_value) f64".to_owned(),
        }
    }

    fn make_c_value(&self, val: &str) -> String {
        match &self {
            GoTypeBasic::Int => format!("int32({})", val),
            GoTypeBasic::Int8 => format!("byte({})", val),
            GoTypeBasic::Int16 => format!("int16({})", val),
            GoTypeBasic::Int32 => format!("int32({})", val),
            GoTypeBasic::Int64 => format!("C.longlong({})", val),
            GoTypeBasic::Uint => format!("uint32({})", val),
            GoTypeBasic::Uint8 => format!("byte({})", val),
            GoTypeBasic::Uint16 => format!("uint16({})", val),
            GoTypeBasic::Uint32 => format!("uint32({})", val),
            GoTypeBasic::Uint64 => format!("C.ulonglong({})", val),
            GoTypeBasic::Uintptr => format!("C.ulonglong({})", val),
            GoTypeBasic::Bool => match val.to_lowercase().as_str() {
                "true" => "int32(0)",
                "false" => "int32(1)",
                _ => panic!("Invalid bool value in make_c_value: {}", val),
            }
            .to_owned(),
            GoTypeBasic::String => format!("C.CString({})", val),
            GoTypeBasic::Byte => format!("byte({})", val),
            GoTypeBasic::Rune => format!("C.CString({})", val),
            GoTypeBasic::Float32 => format!("C.float({})", val),
            GoTypeBasic::Float64 => format!("C.double({})", val),
            _ => panic!("Cannot make C_value from go value of type {}", &self),
        }
        .to_owned()
    }
}

#[derive(Debug, Clone)]
struct GoComment {
    inner: String,
}
impl Serialize for GoComment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Display for GoComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", &self.inner))
    }
}
impl GoComment {
    fn new(s: &str) -> Self {
        let n = s
            .split('\n')
            .map(|line| {
                let mut l = line.trim().to_owned();
                if l.starts_with("/**") {
                    l = l.replacen("/**", "//", 1).trim().to_owned();
                } else if l.starts_with("/*") {
                    l = l.replacen("/*", "//", 1).trim().to_owned();
                } else if l.starts_with("*/") {
                    l = l.replacen("*/", "", 1).trim().to_owned();
                } else if l.starts_with('*') {
                    l = l.replacen("*", "//", 1).trim().to_owned();
                }
                l.trim().to_owned()
            })
            .filter(|line| !line.contains("#meta") && line != "//")
            .collect::<Vec<String>>();

        return GoComment {
            inner: n.join("\n"),
        };
    }
}

#[derive(Clone)]
enum GoTypeComplex {
    Duration,
    Timestamp,
    Url,
    UnsafePointer(GoIdentifier),
    Enum(GoIdentifier),   // GoLabel name for the enum
    Struct(GoIdentifier), // GoLabel name for the struct
    Alias(GoIdentifier),  // GoLabel name for the type alias
    List(Box<GoType>),    // GoLabel data-type for the list
}

impl Serialize for GoTypeComplex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl fmt::Display for GoTypeComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            GoTypeComplex::Duration => "time.Duration".to_owned(),
            GoTypeComplex::Timestamp => "time.Time".to_owned(),
            GoTypeComplex::Url => "url.URL".to_owned(),
            GoTypeComplex::Enum(identifier) => identifier.go_label.to_owned(),
            GoTypeComplex::Struct(identifier) => identifier.go_label.to_owned(),
            GoTypeComplex::Alias(identifier) => identifier.go_label.to_owned(),
            GoTypeComplex::List(data_type) => {
                let s = format!("[]{}", data_type);
                s
            }
            GoTypeComplex::UnsafePointer(identifier) => "unsafe.Pointer".to_owned(),
        };
        f.write_fmt(format_args!("{}", s))
    }
}

impl GoTypeComplex {
    fn make_go_value(&self, val: &str) -> String {
        match &self {
            GoTypeComplex::Duration => format!("time.Duration({} * time.Millisecond)", val),
            GoTypeComplex::Timestamp => format!("time.Unix({}, 0)", val),
            GoTypeComplex::Url => format!("url.Parse({})", val),
            _ => val.to_owned(),
        }
    }

    fn from_c_value(&self, val: &str) -> String {
        match &self {
            GoTypeComplex::Duration => "TODO(nf, not_implemented_from_c_value) duration".to_owned(),
            GoTypeComplex::Timestamp => {
                "TODO(nf, not_implemented_from_c_value) timestamp".to_owned()
            }
            GoTypeComplex::Url => "TODO(nf, not_implemented_from_c_value) url".to_owned(),
            GoTypeComplex::UnsafePointer(_) => {
                "TODO(nf, not_implemented_from_c_value) unsafeptr".to_owned()
            }
            GoTypeComplex::Enum(_) => "TODO(nf, not_implemented_from_c_value) enum".to_owned(),
            GoTypeComplex::Struct(_) => "TODO(nf, not_implemented_from_c_value) struct".to_owned(),
            GoTypeComplex::Alias(_) => "TODO(nf, not_implemented_from_c_value) alis".to_owned(),
            GoTypeComplex::List(_) => "TODO(nf, not_implemented_from_c_value) list".to_owned(),
        }
    }

    fn make_c_value(&self, val: &str) -> String {
        match &self {
            GoTypeComplex::Duration => format!("C.ulonglong({}.Milliseconds())", val),
            GoTypeComplex::Timestamp => format!("C.ulonglong({})", val),
            GoTypeComplex::Url => format!("C.CString({}.String())", val),
            GoTypeComplex::Enum(e) => format!("uint32({})", val),
            GoTypeComplex::Struct(s) => {
                "TODO(nf, not_implemented_make_c_value) struct ??".to_owned()
            }
            GoTypeComplex::Alias(a) => "TODO(nf, not_implemented_make_c_value) alias ??".to_owned(),
            GoTypeComplex::List(_) => "TODO(nf, not_implemented_make_c_value) list ??".to_owned(),
            GoTypeComplex::UnsafePointer(ptr) => format!("unsafe.Pointer({})", ptr),
        }
    }
}

#[derive(Clone)]
enum GoType {
    Void,
    Basic(GoTypeBasic, u8),
    Complex(GoTypeComplex, u8),
}
impl GoType {
    fn requires_free(&self) -> bool {
        match &self {
            GoType::Basic(gtype, _) => match gtype {
                GoTypeBasic::String => true,
                _ => false,
            },
            GoType::Complex(gtype, _) => match gtype {
                GoTypeComplex::Url => true,
                GoTypeComplex::UnsafePointer(_) => true,
                GoTypeComplex::Struct(_) => true,
                GoTypeComplex::List(_) => true,
                _ => false,
            },
            _ => false,
        }
    }
    fn _from(c: &CVariableDeclaration, meta: &MetaValue) -> Self {
        let pcount = if meta.is_nullable {
            std::cmp::max(c.variable_type.pointer_count - 1, 1)
        } else {
            0
        };

        let gt: GoType;

        gt = if meta.is_string {
            GoType::Basic(GoTypeBasic::String, pcount)
        } else if meta.is_url {
            GoType::Complex(GoTypeComplex::Url, pcount)
        } else if meta.is_timestamp {
            GoType::Complex(GoTypeComplex::Timestamp, pcount)
        } else if meta.is_duration {
            GoType::Complex(GoTypeComplex::Duration, pcount)
        } else if meta.is_datetime {
            GoType::Complex(GoTypeComplex::Timestamp, pcount)
        } else if let Some(_) = meta.length_for {
            GoType::Basic(GoTypeBasic::Uint, pcount)
        } else if let Some(_) = meta.capacity_for {
            GoType::Basic(GoTypeBasic::Uint, pcount)
        } else {
            match &c.variable_type.kind {
                CType::Enum(e) => {
                    let complex = GoTypeComplex::Enum(GoIdentifier::new(&e.identifier.label, None));
                    GoType::Complex(complex, pcount)
                }
                CType::SignedShort(_) => GoType::Basic(GoTypeBasic::Int16, pcount),
                CType::UnsignedShort(_) => GoType::Basic(GoTypeBasic::Uint16, pcount),
                CType::SignedInteger(_) => GoType::Basic(GoTypeBasic::Int32, pcount),
                CType::UnsignedInteger(_) => GoType::Basic(GoTypeBasic::Uint32, pcount),
                CType::SignedLong(_) => GoType::Basic(GoTypeBasic::Int64, pcount),
                CType::UnsignedLong(_) => GoType::Basic(GoTypeBasic::Uint64, pcount),
                CType::Int64T(_) => GoType::Basic(GoTypeBasic::Int64, pcount),
                CType::Float(_) => GoType::Basic(GoTypeBasic::Float32, pcount),
                CType::Double(_) => GoType::Basic(GoTypeBasic::Float64, pcount),
                CType::Char(_) => {
                    if c.is_const && c.variable_type.pointer_count > 0 {
                        GoType::Basic(GoTypeBasic::String, pcount)
                    } else {
                        GoType::Basic(GoTypeBasic::Rune, pcount)
                    }
                }
                CType::Struct(s) => {
                    let complex = GoTypeComplex::Enum(GoIdentifier::new(&s.identifier.label, None));
                    GoType::Complex(complex, pcount)
                }
                CType::IntPtrT(_) => GoType::Basic(GoTypeBasic::Int32, pcount),
                CType::UIntPtrT(_) => GoType::Basic(GoTypeBasic::Uintptr, pcount),
                CType::Int8T(_) => GoType::Basic(GoTypeBasic::Int8, pcount),
                CType::Int16T(_) => GoType::Basic(GoTypeBasic::Int64, pcount),
                CType::Int32T(_) => GoType::Basic(GoTypeBasic::Int32, pcount),
                CType::UInt8T(_) => GoType::Basic(GoTypeBasic::Uint8, pcount),
                CType::UInt16T(_) => GoType::Basic(GoTypeBasic::Uint16, pcount),
                CType::UInt32T(_) => GoType::Basic(GoTypeBasic::Uint32, pcount),
                CType::UInt64T(_) => GoType::Basic(GoTypeBasic::Uint64, pcount),
                CType::VoidStar => GoType::Basic(GoTypeBasic::Uintptr, pcount),
                CType::Void => GoType::Void,
                _ => panic!(
                    "Not a valid GoType for given CType. Got: {}",
                    c.variable_type.kind
                ),
            }
        };

        if meta.is_list {
            GoType::Complex(GoTypeComplex::List(Box::new(gt)), pcount)
        } else {
            gt
        }
    }
}
impl From<&CVariableType> for GoType {
    fn from(c: &CVariableType) -> Self {
        match &c.kind {
            CType::Enum(e) => {
                let identifier = GoIdentifier::new(&e.identifier.label, None);
                GoType::Complex(GoTypeComplex::Enum(identifier), c.pointer_count)
            }
            CType::SignedShort(_) => GoType::Basic(GoTypeBasic::Int16, c.pointer_count),
            CType::UnsignedShort(_) => GoType::Basic(GoTypeBasic::Uint16, c.pointer_count),
            CType::SignedInteger(_) => GoType::Basic(GoTypeBasic::Int32, c.pointer_count),
            CType::UnsignedInteger(_) => GoType::Basic(GoTypeBasic::Uint32, c.pointer_count),
            CType::SignedLong(_) => GoType::Basic(GoTypeBasic::Int64, c.pointer_count),
            CType::UnsignedLong(_) => GoType::Basic(GoTypeBasic::Uint64, c.pointer_count),
            CType::Int64T(_) => GoType::Basic(GoTypeBasic::Int64, c.pointer_count),
            CType::Float(_) => GoType::Basic(GoTypeBasic::Float32, c.pointer_count),
            CType::Double(_) => GoType::Basic(GoTypeBasic::Float64, c.pointer_count),
            CType::Char(_) => GoType::Basic(GoTypeBasic::String, c.pointer_count),
            CType::Struct(s) => {
                let complex = GoTypeComplex::Struct(GoIdentifier::new(&s.identifier.label, None));
                GoType::Complex(complex, c.pointer_count)
            }
            CType::IntPtrT(_) => GoType::Basic(GoTypeBasic::Int32, c.pointer_count),
            CType::UIntPtrT(_) => GoType::Basic(GoTypeBasic::Uintptr, c.pointer_count),
            CType::Int8T(_) => GoType::Basic(GoTypeBasic::Int8, c.pointer_count),
            CType::Int16T(_) => GoType::Basic(GoTypeBasic::Int64, c.pointer_count),
            CType::Int32T(_) => GoType::Basic(GoTypeBasic::Int32, c.pointer_count),
            CType::UInt8T(_) => GoType::Basic(GoTypeBasic::Uint8, c.pointer_count),
            CType::UInt16T(_) => GoType::Basic(GoTypeBasic::Uint16, c.pointer_count),
            CType::UInt32T(_) => GoType::Basic(GoTypeBasic::Uint32, c.pointer_count),
            CType::UInt64T(_) => GoType::Basic(GoTypeBasic::Uint64, c.pointer_count),
            CType::VoidStar => GoType::Basic(GoTypeBasic::Uintptr, c.pointer_count),
            CType::Void => GoType::Void,
            _ => panic!("Not a valid GoType for given CType. Got: {}", "<unknown>"),
        }
    }
}
impl From<&CVariableDeclaration> for GoType {
    fn from(c: &CVariableDeclaration) -> Self {
        let meta: MetaValue = MetaValue::from_meta_comment_dontcare(c.comment.to_owned());
        GoType::_from(c, &meta)
    }
}
impl Serialize for GoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl fmt::Display for GoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            GoType::Void => "".to_owned(),
            GoType::Basic(b, pcount) => {
                format!("{}{}", "*".repeat((*pcount).into()), b.to_string())
            }
            GoType::Complex(c, pcount) => {
                format!("{}{}", "*".repeat((*pcount).into()), c.to_string())
            }
        };
        f.write_fmt(format_args!("{}", s))
    }
}

#[derive(Clone)]
struct GoIdentifier {
    go_label: String,
    go_comment: Option<GoComment>,
}
impl Serialize for GoIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: <S as Serializer>::SerializeMap;
        if let Some(c) = &self.go_comment {
            map = serializer.serialize_map(Some(2))?;
            map.serialize_key("go_comment")?;
            map.serialize_value(&c.to_string())?;
        } else {
            map = serializer.serialize_map(Some(1))?;
        }
        map.serialize_key("go_label")?;
        map.serialize_value(&self.go_label)?;

        map.end()
    }
}
impl fmt::Display for GoIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.go_label))
    }
}
impl GoIdentifier {
    fn transform_label(label: &str) -> String {
        let new_label = label.to_owned();
        let mut label_bytes: Vec<u8> = new_label.as_bytes().into();
        let iter = &mut label_bytes.iter().peekable();

        let mut loopidx: usize = 0;
        let mut uppercase_these_indexes: Vec<usize> = vec![];
        while let Some(next) = iter.next() {
            /* always capitalize the first item. This is all public. */
            if loopidx == 0 && next.is_ascii_lowercase() {
                uppercase_these_indexes.push(0)
            }
            if *next == "_".as_bytes()[0] {
                if let Some(peek) = iter.peek() {
                    if peek.is_ascii_lowercase() {
                        uppercase_these_indexes.push(loopidx + 1);
                    }
                }
            }
            loopidx += 1;
        }

        for idx in uppercase_these_indexes {
            label_bytes[idx] = label_bytes[idx].to_ascii_uppercase();
        }
        let mut s = String::from_utf8(label_bytes).unwrap();
        s = s.replace("_", "");
        s
    }

    fn new(label: &str, comment: Option<String>) -> Self {
        let cmt = match comment {
            Some(c) => Some(GoComment::new(&c)),
            None => None,
        };

        return GoIdentifier {
            go_label: Self::transform_label(label),
            go_comment: cmt,
        };
    }
}

#[derive(Serialize, Clone)]
struct GoFunction {
    on_struct: Option<GoIdentifier>,
    identifier: GoIdentifier,
    c_function_name: String,
    go_comment: Option<GoComment>,
    is_void: bool,
    parameters: Vec<GoParameter>,
    meta: MetaValue,
    return_type: GoType,
    return_signature: String,
    is_return_struct: bool,
}

impl GoFunction {
    fn make_return_signature(return_type: &GoType, meta_value: &MetaValue) -> String {
        let s = match return_type {
            GoType::Void if meta_value.throws => "error".to_owned(),
            GoType::Void => "".to_owned(),
            GoType::Basic(_, _) if meta_value.throws => {
                format!("(*{}, error)", return_type.to_string())
            }
            GoType::Complex(_, _) if meta_value.throws => {
                format!("(*{}, error)", return_type.to_string())
            }
            GoType::Complex(_, _) => {
                format!("*{}", return_type.to_string())
            }
            _ => return_type.to_string(),
        };
        s
    }

    fn from_cfunc(all_enums: &Vec<GoEnum>, all_structs: &mut Vec<GoStruct>, c: &CFunction) -> Self {
        let function_meta_values: MetaValue =
            MetaValue::from_meta_comment_dontcare(c.comment.to_owned());

        let mut identifier = GoIdentifier::new(&c.label, None);

        let go_comment = match &c.comment {
            Some(cmt) => Some(GoComment::new(cmt)),
            None => None,
        };
        let mut return_type = GoType::from(&*c.return_type);

        let mut on_struct: Option<GoIdentifier> = None;

        let mut struct_name: Option<&str> = None;

        /* Struct metadata stuff */
        if function_meta_values.for_struct {
            /* retrieve struct name  */
            let mut splits: Vec<&str> = c.label.split("_").into_iter().collect();
            let s_name = *splits.first().unwrap();
            struct_name = Some(s_name);

            /* get new func name (minus struct name) */
            splits.remove(0);
            let func_name = splits.join("_");

            let struct_identifier = GoIdentifier::new(s_name, None);
            on_struct = Some(struct_identifier.clone());

            identifier.go_label = GoIdentifier::transform_label(&func_name);

            /* Return type massaging  */
            if matches!(return_type, GoType::Basic(GoTypeBasic::Uint32, _)) {
                let pcount = match return_type {
                    GoType::Basic(_, pcount) => pcount,
                    _ => panic!("Invalid constructor return type match destruction"),
                };

                /* Constructor specific stuff */
                if function_meta_values.is_constructor {
                    identifier.go_label = format!("{}New", s_name);
                    return_type = GoType::Complex(GoTypeComplex::Struct(struct_identifier), pcount);
                }
            }
        }

        /* Parameter stuff */
        // let param2meta: HashMap<String, Option<&MetaValue>> = HashMap::new();
        let mut param2meta: HashMap<String, MetaValue> = HashMap::new();
        if let Some(cmt) = &c.comment {
            let mut iter = cmt.split('\n').skip_while(|&x| !x.contains("#meta_param"));
            while let Some(c) = iter.next() {
                if !c.contains("#meta_param") {
                    continue;
                }
                let items: Vec<&str> = c.split(';').collect();
                let car = items.first().unwrap().split(':').collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string();
                let cdr = items[1..].join(";");
                let meta_value = MetaValue::from_meta_comment(&cdr);
                param2meta.insert(car, meta_value);
            }
        }

        let mut params: Vec<GoParameter> = vec![];
        for cparam in &c.parameters {
            let meta_option = param2meta.get(&cparam.label);
            let mut gparam = GoParameter::from(cparam, meta_option);
            if let Some(sname) = struct_name {
                /* Attached function parameters */
                if let Some(mv) = meta_option {
                    /* assign As C Field  */
                    if mv.is_this {
                        gparam.as_c_field = format!("{}{}", C_STRUCT_PREFACE, sname);
                    } else if mv.is_output {
                        gparam.as_c_field = match &gparam.go_type {
                            GoType::Complex(c, _) => {
                                format!("{}{}", C_STRUCT_PREFACE, c.to_string())
                            }
                            _ => panic!("cannot modify gparam. Invalid go_type for c_field"),
                        }
                    }

                    // /* Assign From C Field */
                    // gparam.from_c_field = match &gparam.go_type {
                    //     GoType::Void => panic!("Cant make fromcfield from void"),
                    //     GoType::Basic(gtype, _) => {
                    //         gtype.from_c_value(&format!("{}", gparam.c_identifier.label))
                    //     }

                    //     GoType::Complex(gtype, _) => {
                    //         gtype.from_c_value(&format!("{}", gparam.c_identifier.label))
                    //     }
                    // };
                }
            } else {
                /* Free function parameters */
                if cparam.variable_type.is_struct {
                    gparam.as_c_field = format!("{}{}", C_STRUCT_PREFACE, cparam.variable_type.kind)
                }
            }
            params.push(gparam);
        }

        /* Massage return types if not constructor and has output */
        if !function_meta_values.is_constructor && !function_meta_values.is_destructor {
            for param in &params {
                if param.meta.is_output {
                    return_type = param.go_type.clone();
                    break;
                }
            }
        }

        let gf = GoFunction {
            on_struct,
            identifier,
            c_function_name: c.label.to_owned(),
            go_comment,
            return_signature: GoFunction::make_return_signature(
                &return_type,
                &function_meta_values,
            ),
            meta: function_meta_values,
            is_void: matches!(return_type, GoType::Void),
            is_return_struct: matches!(return_type, GoType::Complex(GoTypeComplex::Struct(_), _)),
            parameters: params,
            return_type,
        };

        gf
    }
}

#[derive(Serialize)]
struct GoStruct {
    c_label: String,
    identifier: GoIdentifier,
    go_comment: Option<GoComment>,
    functions: Vec<GoFunction>,
    constructor: Option<GoFunction>,
    fields: Vec<GoField>,
    meta: MetaValue,
}
impl From<&CStruct> for GoStruct {
    fn from(c: &CStruct) -> Self {
        let identifier = GoIdentifier::new(&c.identifier.label, None);
        let c_identifier = CIdentifier::new(&c.identifier.label, None);
        let comment = match &c.comment {
            Some(cmt) => Some(GoComment::new(&cmt)),

            None => None,
        };

        let meta: MetaValue = MetaValue::from_meta_comment_dontcare(c.comment.to_owned());

        let mut fields: Vec<GoField> =
            c.declarations
                .iter()
                .map(|d| {
                    let mut field = GoField::from(d);
                    field.as_c_field = match &field.go_type {
                        GoType::Void => panic!("Cant make gofield c field from void"),
                        GoType::Basic(gtype, _) => gtype
                            .make_c_value(&format!("{}.{}", FIELD_SELF, field.identifier.go_label)),
                        GoType::Complex(gtype, _) => gtype
                            .make_c_value(&format!("{}.{}", FIELD_SELF, field.identifier.go_label)),
                    };
                    field.from_c_field = match &field.go_type {
                        GoType::Void => panic!("Cant make fromcfield from void"),
                        GoType::Basic(gtype, _) => gtype
                            .from_c_value(&format!("{}.{}", FIELD_SELF, field.c_identifier.label)),

                        GoType::Complex(gtype, _) => gtype
                            .from_c_value(&format!("{}.{}", FIELD_SELF, field.c_identifier.label)),
                    };
                    field
                })
                .collect();

        if meta.is_persistent {
            let gf = GoField {
                as_c_field: format!("{}{}", FIELD_SELF, FIELD_PTR),
                from_c_field: "TOFO(NF, IS_PERSISTENT)".to_owned(),
                go_comment: Some(GoComment::new("// reference to C pointer")),
                meta: MetaValue::from_meta_comment_dontcare(None),
                identifier: GoIdentifier {
                    go_label: FIELD_PTR.to_owned(),
                    go_comment: None,
                },
                requires_pointer_dereference: false,
                requires_pointer_reference: true,
                is_complex: true,
                is_struct: true,
                is_list: false,
                c_identifier,
                pointer_count: 0,
                go_type: GoType::Complex(
                    GoTypeComplex::UnsafePointer(GoIdentifier {
                        go_label: FIELD_PTR.to_owned(),
                        go_comment: None,
                    }),
                    0,
                ),
                requires_free: true,
            };
            fields.insert(0, gf);
        }

        let functions: Vec<GoFunction> = vec![];

        GoStruct {
            c_label: c.identifier.label.to_owned(),
            identifier,
            go_comment: comment,
            constructor: None, // is re-assessed later on
            functions,
            fields,
            meta,
        }
    }
}

struct GoEnum {
    identifier: GoIdentifier,
    values: Vec<GoIdentifier>,
    go_comment: Option<GoComment>,
    meta: MetaValue,
}
impl From<&CEnum> for GoEnum {
    fn from(c: &CEnum) -> Self {
        let meta: MetaValue = MetaValue::from_meta_comment_dontcare(c.comment.to_owned());

        let identifier = GoIdentifier::new(&c.identifier.label, None);
        let comment = match &c.comment {
            Some(cmt) => Some(GoComment::new(&cmt)),
            None => None,
        };
        let values: Vec<GoIdentifier> = c
            .declarations
            .iter()
            .map(|m| GoIdentifier::new(&m.label, m.comment.to_owned()))
            .collect();

        GoEnum {
            meta,
            go_comment: comment,
            identifier,
            values,
        }
    }
}
struct _SerdeVecGoIdentifier<'a>(&'a Vec<GoIdentifier>);
impl<'a> Serialize for _SerdeVecGoIdentifier<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for member in self.0.iter() {
            seq.serialize_element(member)?;
        }
        seq.end()
    }
}
impl Serialize for GoEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: <S as Serializer>::SerializeMap;
        if let Some(c) = &self.go_comment {
            map = serializer.serialize_map(Some(3))?;

            map.serialize_entry("go_comment", &c.to_string())?;
        } else {
            map = serializer.serialize_map(Some(2))?;
        }
        map.serialize_entry("identifier", &self.identifier)?;

        map.serialize_key("values")?;
        map.serialize_value(&_SerdeVecGoIdentifier(&self.values))?;
        map.end()
    }
}
impl fmt::Display for GoEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", "HAHAHA"))
    }
}

#[derive(Serialize, Clone)]
struct GoParameter {
    identifier: GoIdentifier,
    c_identifier: CIdentifier,
    go_type: GoType,
    pointer_count: u8,
    is_complex: bool,
    is_struct: bool,
    as_c_field: String,
    from_c_field: String,
    requires_free: bool,
    show_in_signature: bool,
    meta: MetaValue,
}
impl GoParameter {
    fn from(c: &CVariableDeclaration, meta: Option<&MetaValue>) -> Self {
        let meta_value = match meta {
            Some(m) => m.clone(),
            None => MetaValue::new(),
        };
        let identifier = GoIdentifier::new(&c.label, None);
        let c_identifier = CIdentifier::new(&c.label, None);
        let go_type = GoType::_from(c, &meta_value);

        let as_c_field = match &go_type {
            GoType::Void => panic!("Cant make goparameter c field from void"),
            GoType::Basic(gtype, _) => gtype.make_c_value(&format!("{}", identifier.go_label)),
            GoType::Complex(gtype, _) => gtype.make_c_value(&format!("{}", identifier.go_label)),
        };

        GoParameter {
            pointer_count: c.variable_type.pointer_count,
            as_c_field,
            requires_free: meta_value.is_error || go_type.requires_free(),
            show_in_signature: !meta_value.is_this && !meta_value.is_output && !meta_value.is_error,
            meta: meta_value,
            is_complex: matches!(go_type, GoType::Complex(_, _)),
            is_struct: c.variable_type.is_struct,
            from_c_field: match &go_type {
                GoType::Void => panic!("Cant make fromcfield from void"),
                GoType::Basic(gtype, _) => {
                    gtype.from_c_value(&format!("{}{}", C_PREFACE, &identifier.go_label))
                }

                GoType::Complex(gtype, _) => {
                    gtype.from_c_value(&format!("{}{}", C_PREFACE, &identifier.go_label))
                }
            },
            identifier,
            c_identifier,
            go_type,
        }
    }
}

#[derive(Serialize, Clone)]
struct GoField {
    identifier: GoIdentifier,
    c_identifier: CIdentifier,
    go_type: GoType,
    go_comment: Option<GoComment>,
    pointer_count: u8,
    requires_pointer_reference: bool,
    requires_pointer_dereference: bool,
    as_c_field: String,
    from_c_field: String,
    requires_free: bool,
    is_complex: bool,
    is_struct: bool,
    is_list: bool,
    meta: MetaValue,
}

impl From<&CVariableDeclaration> for GoField {
    fn from(c: &CVariableDeclaration) -> Self {
        let go_type = GoType::from(c);
        let go_identifier = GoIdentifier::new(&c.label, None);
        let meta_value = MetaValue::from_meta_comment_dontcare(c.comment.to_owned());
        GoField {
            identifier: go_identifier,
            c_identifier: CIdentifier::new(&c.label, None),
            go_comment: match &c.comment {
                Some(cmt) => Some(GoComment::new(&cmt)),
                None => None,
            },
            requires_pointer_dereference: if meta_value.is_url || c.variable_type.is_struct {
                true
            } else {
                false
            },
            requires_pointer_reference: if meta_value.is_nullable {
                true
            } else if meta_value.is_string {
                if c.variable_type.pointer_count > 1 {
                    true
                } else {
                    false
                }
            } else if meta_value.is_url {
                false // URLs are parsed via url.Parse into a *url.URL by default
            } else {
                c.variable_type.pointer_count > 0 && !meta_value.is_list && !meta_value.is_nullable
            },
            is_struct: c.variable_type.is_struct,
            is_complex: matches!(go_type, GoType::Complex(_, _)),
            is_list: meta_value.is_list,
            requires_free: go_type.requires_free(),
            go_type,
            pointer_count: c.variable_type.pointer_count,
            as_c_field: "".to_owned(),
            from_c_field: "".to_owned(),
            meta: meta_value,
        }
    }
}

#[derive(Serialize)]
struct GoDeclaration {
    identifier: GoIdentifier,
    go_type: GoType,
    go_value: Option<String>,
    go_comment: Option<GoComment>,
}
impl fmt::Display for GoDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.go_value {
            None => f.write_fmt(format_args!("var {} {}", self.identifier, self.go_type)),
            Some(go_val) => {
                let val = match &self.go_type {
                    GoType::Basic(b, _) => b.make_go_value(&go_val),
                    GoType::Complex(c, _) => c.make_go_value(&go_val),
                    _ => panic!(
                        "Received invalid declaration type. Cannot format GoDeclaration of type {}",
                        &self.go_type,
                    ),
                };
                f.write_fmt(format_args!(
                    "{} {} = {}",
                    self.identifier, self.go_type, val
                ))
            }
        }
    }
}

#[derive(Serialize)]

struct GoConstant {
    comment: Option<String>,
}

#[derive(Serialize)]
struct DataGoInformation {
    package_name: String,
    ld_flags: String,
    header_file_location: String,
}

#[derive(Serialize)]
struct Data {
    go_enums: Vec<GoEnum>,
    go_structs: Vec<GoStruct>,
    go_functions: Vec<GoFunction>,
    meta: DataGoInformation,
}
impl Data {
    fn new(
        header: &HeaderFile,
        package_name: &str,
        ld_flags: &str,
        header_file_location: &str,
    ) -> Self {
        let go_enums: Vec<GoEnum> = header.enums.iter().map(|e| GoEnum::from(e)).collect();
        let mut go_structs: Vec<GoStruct> =
            header.structs.iter().map(|s| GoStruct::from(s)).collect();
        let mut go_functions: Vec<GoFunction> = header
            .functions
            .iter()
            .map(|f| GoFunction::from_cfunc(&go_enums, &mut go_structs, f))
            .collect();

        /* attach to structs as necessary */
        for gfunc in &go_functions {
            if let Some(n) = &gfunc.on_struct {
                for gstruct in &mut go_structs {
                    if gstruct.identifier.go_label == n.go_label {
                        let gfclone = gfunc.clone();
                        if gfunc.meta.is_constructor {
                            gstruct.constructor = Some(gfclone);
                        } else {
                            gstruct.functions.push(gfclone);
                        }
                        break;
                    }
                }
            }
        }
        go_functions.retain(|f| matches!(f.on_struct, None));

        return Data {
            go_enums,
            go_structs,
            go_functions,
            meta: DataGoInformation {
                package_name: package_name.to_owned(),
                ld_flags: ld_flags.to_owned(),
                header_file_location: header_file_location.to_owned(),
            },
        };
    }
}

/// Returns true if `value` is Some(_)
fn is_some(value: Option<&Value>, _: &[Value]) -> tera::Result<bool> {
    match value {
        Some(Value::Null) | None => Ok(false),
        _ => Ok(true),
    }
}

pub fn generate(
    header: HeaderFile,
    package_name: &str,
    ld_flags: &str,
    header_file_location: &str,
) -> String {
    let data = Data::new(
        &header,
        package_name,
        ld_flags,
        header_file_location,
        // header.defines.iter().map(|cv| Variable::from(cv)).collect(),
        // header.enums.iter().map(|e| Enum::from(e)).collect(),
        // &mut header.structs.iter().map(|s| Struct::from(s)).collect(),
        // &mut header.functions.iter().map(|f| Function::from(f)).collect(),
    );
    let mut context = Context::new();
    context.insert("enums", &data.go_enums);
    context.insert("structs", &data.go_structs);
    context.insert("functions", &data.go_functions);
    context.insert("go_data", &data.meta);

    let mut tera = Tera::default();
    tera.register_tester("some", is_some);

    tera.add_raw_templates(vec![
        ("generated_header", TEMPLATE_GENERATED_HEADER),
        ("go_header", TEMPLATE_GO_HEADER),
        ("enum_definitions", TEMPLATE_ENUM_DEFINITIONS),
        ("enum_declarations", TEMPLATE_ENUM_DECLARATIONS),
        (
            "struct_inner_declarations",
            TEMPLATE_STRUCT_INNER_DECLARATIONS,
        ),
        ("from_c_structs", TEMPLATE_FROM_C_STRUCT),
        (
            "struct_external_definitions",
            TEMPLATE_STRUCT_EXTERNAL_DECLARATIONS,
        ),
        ("free_functions", TEMPLATE_FREE_FUNCTIONS),
        ("struct_functions", TEMPLATE_STRUCT_FUNCTIONS),
        ("struct_constructor", TEMPLATE_STRUCT_CONSTRUCTOR),
        ("extender", TEMPLATE_EXTENDER),
        ("helpers", TEMPLATE_HELPERS),
    ])
    .unwrap();

    let mut s: String = "".to_owned();
    s = format!(
        "{}{}",
        s,
        tera.render("generated_header", &context).unwrap()
    );
    s = format!("{}{}", s, tera.render("go_header", &context).unwrap());
    s = format!(
        "{}{}",
        s,
        tera.render("enum_declarations", &context).unwrap()
    );
    s = format!(
        "{}{}",
        s,
        tera.render("enum_definitions", &context).unwrap()
    );
    s = format!(
        "{}{}",
        s,
        tera.render("struct_inner_declarations", &context).unwrap()
    );
    s = format!("{}{}", s, tera.render("from_c_structs", &context).unwrap());
    s = format!(
        "{}{}",
        s,
        tera.render("struct_external_definitions", &context)
            .unwrap()
    );
    s = format!(
        "{}{}",
        s,
        tera.render("struct_constructor", &context).unwrap()
    );
    s = format!(
        "{}{}",
        s,
        tera.render("struct_functions", &context).unwrap()
    );

    s = format!("{}{}", s, tera.render("free_functions", &context).unwrap());
    s = format!("{}{}", s, tera.render("helpers", &context).unwrap());
    s
}

const TEMPLATE_HELPERS: &'static str = "
func deserializeTime(ts C.longlong) time.Time {
	var t time.Time
	if ts > 0 {
		l := int64(ts)
		t = time.Unix(l, 0)
	}
	return t
}

func _fromListStruct(arr **C.char, length uint, capacity uint) []string {
	var go_Arr []string
	_c_elements := (*[1 << 30]*C.char)(unsafe.Pointer(arr))[length:length]
	if _c_elements != nil {
		for _, _c_iter_item := range _c_elements {
			go_iter_item := C.GoString(_c_iter_item)
			go_Arr = append(go_Arr, go_iter_item)
		}
	}
	return go_Arr
}
";

const TEMPLATE_EXTENDER: &'static str =
    "{% extends \"struct_functions\" %}{% block hey %}Parent{% endblock hey %}";

const TEMPLATE_GENERATED_HEADER: &'static str = "
// DO NOT MODIFY THIS FILE
// This file contains automatically generated Go Bindings.
// It was generated via the clang2src project, and ultimately comes from a set of annotated Rust source files
// Any modifications you make tos this file will be reverted whenever this file is regenerated
";

const TEMPLATE_GO_HEADER: &'static str = "
package {{go_data.package_name}}

/*
#cgo CFLAGS: -g -Wall
#cgo LDFLAGS: {{go_data.ld_flags}}
#include \"{{go_data.header_file_location}}\"
*/
import \"C\"

import (
	\"errors\"
	\"net/url\"
	\"time\"
	\"unsafe\"
)
";

const TEMPLATE_ENUM_DECLARATIONS: &'static str = "
{% block title %}{% endblock %}
{% if enums|length %}
// Publically declared Enums
type (
    {% for enum in enums %}
        {% if enum.go_comment is defined %}{{ enum.go_comment }}{% endif %}
        {{ enum.identifier.go_label }} int
        {% endfor %}
)
{% endif %}
";

const TEMPLATE_ENUM_DEFINITIONS: &'static str = "
{% block title %}{% endblock %}
{% if enums|length %}
// Enum iota definitions
    {% for enum in enums %}
        const (
            {% for value in enum.values %}
                {% if value.go_comment is defined %}{{ value.go_comment }}{% endif %}
                {{ value.go_label }}{% if loop.first %} {{ enum.identifier.go_label}} = iota {% endif %}
            {% endfor %}
        )
    {% endfor %}
{% endif %}
";

const TEMPLATE_STRUCT_INNER_DECLARATIONS: &'static str = "
{% block title %}{% endblock %}
{% if structs|length %}
// Struct private mapping declarations
type (
    {% for struct in structs %}
    _{{struct.identifier.go_label}} C.struct_{{struct.c_label}}
    {% endfor %}
)
{% endif %}
";

const TEMPLATE_FROM_C_STRUCT: &'static str = "
{% if structs|length %}
{% for struct in structs|filter(attribute=\"meta.is_persistent\", value=false) %}
func _{{struct.identifier.go_label}}FromCStruct(_obj *C.struct_{{struct.c_label}}) (*{{struct.identifier.go_label}}, error) {
    {% for field in struct.fields %}
        {% if field.is_complex %}
        var go_{{field.identifier.go_label}}  {% if field.pointer_count > 0  and not field.meta.is_list%}*{% endif %}{{field.go_type}}
        {% if field.go_type == \"time.Time\" %}
        go_{{field.identifier.go_label}} = deserializeTime(_obj.{{field.c_identifier.label}})
        {% elif field.go_type == \"url.URL\" %}
        go_{{field.identifier.go_label}}, err := url.Parse(C.GoString(_obj.{{field.c_identifier.label}}))
        if err != nil {
            return nil, fmt.Errorf(\"Failed to get URL: Received invalid URL: %s\", err.Error())
        }
        {% elif field.is_struct %}
        if _obj.{{field.c_identifier.label}} != nil {
            go_{{field.identifier.go_label}}NotNil, err := _{{field.go_type}}FromCStruct(_obj.{{field.c_identifier.label}})
            if err != nil {
                return nil, fmt.Errorf(\"Failed to get struct: %s\", err.Error())
            }
            go_{{field.identifier.go_label}} = go_{{field.identifier.go_label}}NotNil
        }
        {% elif field.meta.is_list %}
        go_{{field.identifier.go_label}} = _fromListStruct(_obj.{{field.c_identifier.label}}, go_len_arr, go_capacity_arr)        
        {% endif %}
        {% else %}
        {% if field.meta.length_for is some %}
        go_len_{{field.meta.length_for}} := {{field.from_c_field}}
        {% elif field.meta.capacity_for is some %}
        go_capacity_{{field.meta.capacity_for}} := {{field.from_c_field}}
        {% else %}
        {% if field.meta.is_nullable %}
        var go_{{field.identifier.go_label}} {{field.go_type}}
        if _obj.{{field.c_identifier.label}} != nil {
            _assign_{{field.identifier.go_label}} := {{field.from_c_field}}
            go_{{field.identifier.go_label}} = &_assign_{{field.identifier.go_label}}
        }
        {% else %}
        go_{{field.identifier.go_label}} := {{field.from_c_field}}
        {% endif %}
        {% endif %}
        {% endif %}
    {% endfor %}

    _res := {{struct.identifier.go_label}} {
        {% for field in struct.fields %}
        {{field.identifier.go_label}}: {%if field.requires_pointer_dereference %}*{% elif field.requires_pointer_reference %} &{% endif %}{% if field.meta.capacity_for is some %}go_capacity_{{field.meta.capacity_for}}
        {% elif field.meta.length_for is some %}go_len_{{field.meta.length_for}}{% else %}go_{{field.identifier.go_label}}{% endif %},
        {% endfor %}
    }

    return &_res, nil
}
{% endfor %}
{% endif %}";
const TEMPLATE_STRUCT_EXTERNAL_DECLARATIONS: &'static str = "
{% block title %}{% endblock %}
{% if structs|length %}
{% for struct in structs %}
{% if struct.go_comment is defined %}{{ struct.go_comment }}{% endif %}
type {{ struct.identifier.go_label }} struct {
    {% for field in struct.fields %}
    {% if field.go_comment is defined %}{{ field.go_comment }}{% endif %}
    {{ field.identifier.go_label }} {{ field.go_type }}
    {% endfor %}
}
{% endfor %}
{% endif %}
";

const TEMPLATE_STRUCT_CONSTRUCTOR: &'static str = "
{% block title %}{% endblock %}
{% for struct in structs %}
{% if struct.constructor is some %}
func {{struct.constructor.identifier.go_label}}({% for param in struct.constructor.parameters|filter(attribute=\"show_in_signature\", value=true) %}{{param.identifier.go_label}} {{param.go_type}}, {% endfor %}) {{struct.constructor.return_signature}} {
    
    // make C Objects
    {% for param in struct.constructor.parameters %}
    {% if param.meta.is_this %}
    var c_This *{{param.as_c_field}}
    {% elif param.meta.is_error %}
    c_{{param.identifier.go_label}} := C.CString(\"\")
    {% else %}
    c_{{param.identifier.go_label}} := {{param.as_c_field}}
    {% endif %}
    {% endfor %}

    {% if struct.constructor.parameters|filter(attribute=\"requires_free\", value=true)|filter(attribute=\"meta.is_this\", value=false)|length %}
    // free C-objects

    defer func(_ptrs []unsafe.Pointer) {
		for _, p := range _ptrs {
			C.free(p)
		}
	}([]unsafe.Pointer{ {% for param in struct.constructor.parameters | filter(attribute=\"requires_free\", value=true)|filter(attribute=\"meta.is_this\", value=false) %}unsafe.Pointer(c_{{param.identifier.go_label}}), {% endfor %} })
    
    {% endif %}

    {% if struct.constructor.meta.throws %}
    if C.{{struct.constructor.c_function_name}}({% for param in struct.constructor.parameters %}
        {% if param.meta.is_this or param.meta.is_output or param.meta.is_error %}&{% endif %}{% if param.meta.is_this %} c_This {% else %} c_{{param.identifier.go_label}} {% endif %}, 
        {% endfor %}) != C.OAUTHTOOL_PASS {
        return nil, errors.New(C.GoString({% for err_param in struct.constructor.parameters|filter(attribute=\"meta.is_error\", value=true) %}c_{{err_param.identifier.go_label}}{% endfor %}))
    }
    go_ret := {{struct.identifier.go_label}} {
        ptr: unsafe.Pointer(c_This),
    }
    return &go_ret, nil
    {% else %}
    // TODO(nf, struct_constructor): only handles functions that return pointers to output. Does not handle passing parameter pointers-to-be-filled
    _res_from_c := C.{{struct.constructor.c_function_name}}({% for param in struct.constructor.parameters %}
        {% if param.meta.is_output or param.meta.is_error %}&{% endif %}c_{{param.identifier.go_label}}, 
        {% endfor %})
    _res_s, err := _{{struct.constructor.return_type}}FromCStruct(_res_from_c)
    {% endif %}
}

{% endif %}
{% endfor %}
";

const TEMPLATE_STRUCT_FUNCTIONS: &'static str = "
{% for struct in structs %}
{% for function in struct.functions %}
{% if function.go_comment is defined %}{{ function.go_comment }}{% endif %}
func (_obj *{{struct.identifier.go_label}}) {{function.identifier.go_label}}({% for param in function.parameters|filter(attribute=\"show_in_signature\", value=true) %}{{param.identifier.go_label}} {{param.go_type}}, {% endfor %}) {{function.return_signature}} {
    // make C Objects
    {% for param in function.parameters %}
    {% if param.meta.is_this %}
    c_This := (*C.struct_{{struct.identifier.go_label}})(_obj.ptr)
    {% elif param.meta.is_output %}
        {% if param.is_complex %}
    var c_{{param.identifier.go_label}} *{{param.as_c_field}}
        {% else %}
    c_{{param.identifier.go_label}} := C.CString(\"\")
        {% endif %}
    {% elif param.meta.is_error %}
    c_{{param.identifier.go_label}} := C.CString(\"\")
    {% else %}
    c_{{param.identifier.go_label}} := {{param.as_c_field}}
    {% endif %}
    {% endfor %}

    {% if function.parameters|filter(attribute=\"requires_free\", value=true)|filter(attribute=\"meta.is_this\", value=false)|length %}
    // free C-objects
    defer func(_ptrs []unsafe.Pointer) {
		for _, p := range _ptrs {
			C.free(p)
		}
	}([]unsafe.Pointer{ {% for param in function.parameters | filter(attribute=\"requires_free\", value=true)|filter(attribute=\"meta.is_this\", value=false) %}unsafe.Pointer(c_{{param.identifier.go_label}}), {% endfor %} })
    
    {% endif %}


    {% if function.meta.throws %}
    if C.{{function.c_function_name}}({% for param in function.parameters %}
        {% if param.meta.is_output or param.meta.is_error %}&{% endif %}{% if param.meta.is_this %} c_This {% else %} c_{{param.identifier.go_label}} {% endif %}, 
        {% endfor %}) != C.OAUTHTOOL_PASS {
        return nil, errors.New(C.GoString({% for err_param in function.parameters|filter(attribute=\"meta.is_error\", value=true) %}c_{{err_param.identifier.go_label}}{% endfor %}))
    }

    {% for param in function.parameters|filter(attribute=\"meta.is_output\", value=true) %}
        {% if param.is_struct %}
    _res_{{param.identifier.go_label}}, err := _{{param.go_type}}FromCStruct(c_{{param.identifier.go_label}})
    if err != nil {
        return nil, fmt.Errorf(\"Failed to create cstruct for item {{param.identifier.go_label}}: %s\", err.Error())
    }
        {% else %}
    _res_{{param.identifier.go_label}} := {{param.as_c_field}}
        {% endif %}
    return _res_{{param.identifier.go_label}}, nil
    {% endfor %}
    {% else %}
    C.{{function.c_function_name}}()
    return TODO(NF, fix for non-throws return values), nil
    {% endif %}
}
{% endfor %}
{% endfor %}
";

const TEMPLATE_FREE_FUNCTIONS: &'static str = "
{% block title %}{% endblock %}
{% if functions|length %}
{% for function in functions %}
{% if function.go_comment is defined %}{{ function.go_comment }}{% endif %}
func {{function.identifier.go_label}}({% for param in function.parameters|filter(attribute=\"show_in_signature\", value=true) %}{{param.identifier.go_label}} {{param.go_type}}, {% endfor %}) {{function.return_signature}} {
    // make C Objects
    {% for param in function.parameters %}
        {% if param.meta.is_output %}
            {% if param.is_complex %}
    var c_{{param.identifier.go_label}} *{{param.as_c_field}}
            {% else %}
    c_{{param.identifier.go_label}} := C.CString(\"\")
            {% endif %}
        {% elif param.meta.is_error %}
    c_{{param.identifier.go_label}} := C.CString(\"\")
        {% else %}
            {% if param.is_struct %}
            // 	var c_Engine *C.struct_HubkitEngine = (*C.struct_HubkitEngine)(Engine.ptr)
    c_{{param.identifier.go_label}} := (*C.struct_{{param.go_type}})({{param.identifier.go_label}}.ptr)
            {% elif param.is_complex %}
    var c_{{param.identifier.go_label}} *{{param.as_c_field}}
            {% else %}
    c_{{param.identifier.go_label}} := {{param.as_c_field}}
            {% endif %}
        {% endif %}
    {% endfor %}

    {% if function.parameters|filter(attribute=\"requires_free\", value=true)|filter(attribute=\"meta.is_this\", value=false)|length %}
    // free C-objects

    defer func(_ptrs []unsafe.Pointer) {
		for _, p := range _ptrs {
			C.free(p)
		}
	}([]unsafe.Pointer{ {% for param in function.parameters | filter(attribute=\"requires_free\", value=true)|filter(attribute=\"meta.is_this\", value=false) %}unsafe.Pointer(c_{{param.identifier.go_label}}), {% endfor %} })
    {% endif %}


    {% if function.meta.throws %}
    if C.{{function.c_function_name}}({% for param in function.parameters %}
        {% if param.meta.is_output or param.meta.is_error %}&{% endif %}{% if param.meta.is_this %} c_This {% else %} c_{{param.identifier.go_label}} {% endif %}, 
        {% endfor %}) != C.OAUTHTOOL_PASS {
        return nil, errors.New(C.GoString({% for err_param in function.parameters|filter(attribute=\"meta.is_error\", value=true) %}c_{{err_param.identifier.go_label}}{% endfor %}))
    }
        {% for param in function.parameters|filter(attribute=\"meta.is_output\", value=true) %}
            {% if param.is_struct %}
    _res_{{param.identifier.go_label}}, err := _{{param.go_type}}FromCStruct(c_{{param.identifier.go_label}})
    if err != nil {
        return nil, fmt.Errorf(\"Failed to create cstruct for item {{param.identifier.go_label}}: %s\", err.Error())
    }
            {% else %}
    _res_{{param.identifier.go_label}} := {{param.from_c_field}}
            {% endif %}
    return {% if not param.is_complex %}&{% endif %}_res_{{param.identifier.go_label}}, nil
        {% endfor %}
    {% else %}
    // TODO(nf, free_funtions): only handles functions that return pointers to output. Does not handle passing parameter pointers-to-be-filled
    _res_from_c := C.{{function.c_function_name}}({% for param in function.parameters %}
        {% if param.meta.is_output or param.meta.is_error %}&{% endif %}c_{{param.identifier.go_label}}, 
        {% endfor %})
    _res_s, err := _{{function.return_type}}FromCStruct(_res_from_c)
    {% endif %}
}
{% endfor %}
{% endif %}
";
