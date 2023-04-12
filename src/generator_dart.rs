use core::fmt::Display;
use core::panic;
use serde::ser::{SerializeMap, SerializeSeq};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Formatter;
use tera::{Context, Tera};

use serde::{Serialize, Serializer};

use crate::lexer::{
    CEnum, CFunction, CIdentifier, CStruct, CType, CVariableDeclaration, CVariableType, HeaderFile,
};
use crate::meta::{MetaValue, META_TOKEN};

const C_PREFIX: &str = "C_";

#[derive(Serialize)]
struct Data<'a> {
    library_path: &'a str,
    library_name: &'a str,
    usings: Vec<&'a str>,
    enums: Vec<DartEnum>,
    ffi_structs: Vec<DartFFIStruct>,
    // constants: Vec<DartVariable>,
    // enums: Vec<CSharpEnum>,
    // structs: Vec<CSharpStruct>,
    // functions: Vec<CSharpFunction>,
}

impl<'a> Data<'a> {
    fn new(
        header: &HeaderFile,
        library_path: &'a str,
        library_name: &'a str,
        usings: Vec<&'a str>,
    ) -> Self {
        let enums: Vec<DartEnum> = header.enums.iter().map(DartEnum::from).collect();
        let ffi_structs: Vec<DartFFIStruct> =
            header.structs.iter().map(DartFFIStruct::from).collect();
        Data {
            library_path,
            library_name,
            usings,
            enums,
            ffi_structs,
        }
    }

    fn make_tera_context(&self) -> tera::Context {
        let mut context = Context::new();
        context.insert(
            "meta",
            &DataDartInformation {
                library_name: self.library_name.to_owned(),
                library_path: self.library_path.to_owned(),
            },
        );
        context.insert("usings", &self.usings);
        context.insert("enums", &self.enums);
        context.insert("ffi_structs", &self.ffi_structs);

        context
    }
}

struct _SerdeVecDartIdentifier<'a>(&'a Vec<DartIdentifier>);
impl<'a> Serialize for _SerdeVecDartIdentifier<'a> {
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

#[derive(Serialize)]
struct DataDartInformation {
    library_path: String,
    library_name: String,
}

#[derive(Debug, Clone)]
struct DartComment {
    inner: String,
}
impl Serialize for DartComment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for DartComment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.inner))
    }
}

impl From<String> for DartComment {
    fn from(s: String) -> Self {
        let n = s
            .split('\n')
            .map(|line| {
                let mut l = line.trim().to_owned();
                if l.starts_with("/**") {
                    l = l.replacen("/**", "///", 1).trim().to_owned();
                } else if l.starts_with("/*") {
                    l = l.replacen("/*", "///", 1).trim().to_owned();
                } else if l.starts_with("*/") {
                    l = l.replacen("*/", "", 1).trim().to_owned();
                } else if l.starts_with('*') {
                    l = l.replacen("*", "///", 1).trim().to_owned();
                }
                l.trim().to_owned()
            })
            .filter(|line| !line.contains(META_TOKEN) && line != "//" && line != "///")
            .collect::<Vec<String>>();

        return DartComment {
            inner: n.join("\n"),
        };
    }
}

#[derive(Debug, Clone)]
struct DartIdentifier {
    dart_label: String,
    dart_comment: Option<DartComment>,
}
impl Serialize for DartIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: <S as Serializer>::SerializeMap;
        if let Some(c) = &self.dart_comment {
            map = serializer.serialize_map(Some(2))?;
            map.serialize_key("dart_comment")?;
            map.serialize_value(&c.to_string())?;
        } else {
            map = serializer.serialize_map(Some(1))?;
        }
        map.serialize_key("dart_label")?;
        map.serialize_value(&self.dart_label)?;

        map.end()
    }
}
impl Display for DartIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.dart_label))
    }
}

impl DartIdentifier {
    /// Ensures that the label for this type starts with a capital
    fn make_label_for_custom_type(label: &str) -> String {
        let mut l = DartIdentifier::transform_label(label);
        unsafe {
            let lbytes = l.as_bytes_mut();
            if let Some(first) = lbytes.first() {
                if first.is_ascii_lowercase() {
                    lbytes[0] = first.to_ascii_uppercase();
                }
            }
        }
        l
    }

    /// Ensures that the variables is public, and conforms to Dart standards
    fn transform_label(label: &str) -> String {
        let underscore_bytes = "_".as_bytes()[0];
        let new_label = label.to_owned();
        let mut label_bytes: Vec<u8> = new_label.as_bytes().into();
        let iter = &mut label_bytes.iter().peekable();

        let mut loopidx: usize = 0;
        let mut uppercase_these_indexes: Vec<usize> = vec![];
        let mut lowercase_these_indexes: Vec<usize> = vec![];
        while let Some(next) = iter.next() {
            /* Remove leading underscore and ensure next item is lowercased. This is all public. */
            if loopidx == 0 && *next == underscore_bytes {
                lowercase_these_indexes.push(loopidx + 1)
            } else if loopidx == 0 && next.is_ascii_uppercase() {
                lowercase_these_indexes.push(0);
            } else {
                if *next == underscore_bytes {
                    if let Some(peek) = iter.peek() {
                        if peek.is_ascii_lowercase() {
                            uppercase_these_indexes.push(loopidx + 1);
                        }
                    }
                }
            }
            loopidx += 1;
        }

        for idx in uppercase_these_indexes {
            label_bytes[idx] = label_bytes[idx].to_ascii_uppercase();
        }
        for idx in lowercase_these_indexes {
            label_bytes[idx] = label_bytes[idx].to_ascii_lowercase();
        }
        let mut s = String::from_utf8(label_bytes).unwrap();
        s = s.replace("_", "");
        s
    }

    fn new(label: &str, comment: Option<String>) -> Self {
        let dart_comment = comment.map(DartComment::from);

        return DartIdentifier {
            dart_label: Self::transform_label(label),
            dart_comment,
        };
    }
}

#[derive(Clone, Serialize)]
struct DartEnum {
    name: String,
    values: Vec<DartEnumOption>,
    dart_comment: Option<DartComment>,
    meta: MetaValue,
}

impl From<&CEnum> for DartEnum {
    fn from(c: &CEnum) -> Self {
        let meta: MetaValue = MetaValue::from_meta_comment_dontcare(&c.comment);

        let enum_name = DartIdentifier::make_label_for_custom_type(&c.identifier.label);
        let dart_comment = c.comment.to_owned().map(DartComment::from);

        let values: Vec<DartEnumOption> = c
            .declarations
            .iter()
            .enumerate()
            .map(|(idx, ident)| DartEnumOption::new(&ident.label, ident.comment.to_owned(), idx))
            .collect();

        DartEnum {
            name: enum_name,
            dart_comment,
            values,
            meta,
        }
    }
}
#[derive(Clone, Serialize)]
struct DartEnumOption {
    /// Enum Option name
    /// e.g., 'S256'
    label: String,
    dart_comment: Option<DartComment>,

    /// To traverse across C, can only be integers
    value: usize,
}
impl DartEnumOption {
    fn new(label: &str, comment: Option<String>, value: usize) -> Self {
        DartEnumOption {
            label: label.to_owned(),
            dart_comment: comment.map(DartComment::from),
            value,
        }
    }
}

/// A field, as in a member of a class
#[derive(Serialize, Clone)]
struct DartField {
    /// The actual identifier for this item
    /// e.g., 'access_tolen'
    identifier: DartIdentifier,

    /// Comments on this field
    comment: Option<DartComment>,

    /// Mostly for FFI
    /// e.g., '@ffi.UintPtr()`
    annotations: Vec<String>,

    /// Mostly for FFI
    /// e..g, 'external'
    modifiers: Vec<String>,

    /// Dart datatype
    /// e.g., 'int', 'Pointer<Char>', etc
    kind: DartDataType,
}

/// A parameter, as in an argument to a function
#[derive(Serialize, Clone)]
struct DartParameter {}

struct DartVariable {
    label: String,
    c_comment: Option<String>,
    dart_comment: Option<String>,
    // value: Option<DataValue>,
    data_type: DartNativeDataType,
    pointer_count: u8,
    is_last: bool,
    meta: MetaValue,
}

impl Display for DartVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stars: String = "".into();
        for _ in 0..self.pointer_count {
            stars += "*";
        }

        f.write_fmt(format_args!("{}{}", self.data_type, stars))
    }
}

/// The FFI backing structure for a Dart class
#[derive(Serialize)]
struct DartFFIStruct {
    /// sets `extends ffi.Opaque` if true
    /// which is true if this struct has no fields
    /// otherwise sets `extends ffi.Struct`
    is_opaque: bool,

    /// The CamelCase name of this struct
    /// Takes the form of C_{Label}
    label: String,

    /// Member fields on this FFI struct
    /// Annoations will be dealt with at template generation time
    fields: Vec<DartField>,

    comment: Option<DartComment>,
}

impl From<&CStruct> for DartFFIStruct {
    fn from(c: &CStruct) -> Self {
        let comment = c.comment.to_owned().map(DartComment::from);

        let fields: Vec<DartField> = c
            .declarations
            .iter()
            .map(|decl| {
                let ffi_kind = DartFFIDataType::from(&decl.variable_type);
                DartField {
                    identifier: DartIdentifier::new(&decl.label, None),
                    comment: decl.comment.to_owned().map(DartComment::from),
                    annotations: vec![ffi_kind.get_dart_annotation_string()],
                    modifiers: vec!["external".to_owned()],
                    kind: DartDataType::FFIType(ffi_kind),
                }
            })
            .collect();
        DartFFIStruct {
            is_opaque: c.declarations.len() == 0,
            label: format!(
                "{}{}",
                C_PREFIX,
                DartIdentifier::make_label_for_custom_type(&c.identifier.label)
            ),
            comment,
            fields,
        }
    }
}

#[derive(Debug, Clone)]
enum DartFFIDataType {
    Pointer { sub_type: Box<DartFFIDataType> },
    NativeType,
    Opaque(DartIdentifier),
    Struct(DartIdentifier),
    Handle,
    NativeFunction { sub_type: Box<DartFFIDataType> },
    Char,
    Void,
    Int8,
    Int16,
    Int32,
    Int64,
    UIntPtr,
    UInt8,
    Uint16,
    UInt32,
    UInt64,
    Float,
    Double,
    Bool,
}

impl Serialize for DartFFIDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<&CType> for DartFFIDataType {
    fn from(c: &CType) -> Self {
        match &c {
            CType::Enum(_)
            | CType::Include(_)
            | CType::Void
            | CType::VoidStar
            | CType::Define(_, _)
            | CType::Function(_)
            | CType::UNINITIALIZED => {
                panic!("Cannot create a Dart FFI Type from this ctype: {}", c)
            }
            CType::Struct(c) => DartFFIDataType::Struct(DartIdentifier::new(
                &DartIdentifier::make_label_for_custom_type(&c.identifier.label),
                None,
            )),
            CType::SignedShort(_) | CType::Int8T(_) => DartFFIDataType::Int8,
            CType::UnsignedShort(_) | CType::UInt8T(_) => DartFFIDataType::UInt8,
            CType::SignedInteger(_) | CType::Int16T(_) => DartFFIDataType::Int16,
            CType::UnsignedInteger(_) | CType::UInt16T(_) => DartFFIDataType::Uint16,
            CType::SignedLong(_) | CType::Int32T(_) => DartFFIDataType::Int32,
            CType::UnsignedLong(_) | CType::UInt32T(_) => DartFFIDataType::UInt32,
            CType::Float(_) => DartFFIDataType::Float,
            CType::Double(_) | CType::DoubleDouble(_) => DartFFIDataType::Double,
            CType::Char(_) => DartFFIDataType::Char,
            CType::IntPtrT(_) | CType::Int64T(_) | CType::UInt64T(_) => DartFFIDataType::Int64,
            CType::UIntPtrT(_) => DartFFIDataType::UIntPtr,
        }
    }
}

impl From<&CVariableType> for DartFFIDataType {
    fn from(c: &CVariableType) -> Self {
        let mut pointer_count = c.pointer_count;
        let mut ffi_type = DartFFIDataType::from(&c.kind);
        while pointer_count > 0 {
            ffi_type = DartFFIDataType::Pointer {
                sub_type: Box::new(ffi_type),
            };
            pointer_count -= 1;
        }
        ffi_type
    }
}

impl DartFFIDataType {
    fn get_dart_annotation_string(&self) -> String {
        match &self {
            DartFFIDataType::NativeType
            | DartFFIDataType::Opaque(_)
            | DartFFIDataType::Struct(_)
            | DartFFIDataType::Handle
            | DartFFIDataType::NativeFunction { sub_type: _ }
            | DartFFIDataType::Pointer { sub_type: _ }
            | DartFFIDataType::Void => "".to_owned(),
            DartFFIDataType::Char
            | DartFFIDataType::Int64
            | DartFFIDataType::UIntPtr
            | DartFFIDataType::Int8
            | DartFFIDataType::Int16
            | DartFFIDataType::Int32
            | DartFFIDataType::UInt8
            | DartFFIDataType::Uint16
            | DartFFIDataType::UInt32
            | DartFFIDataType::UInt64
            | DartFFIDataType::Float
            | DartFFIDataType::Double
            | DartFFIDataType::Bool => {
                format!("@ffi.{}()", self.to_string())
            }
        }
    }
}

impl Display for DartFFIDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DartFFIDataType::Char => f.write_str("ffi.Char"),
            DartFFIDataType::Void => f.write_str("ffi.Void"),
            DartFFIDataType::NativeType => f.write_str("ffi.NativeType"),
            DartFFIDataType::Opaque(ident) => f.write_str("ffi.Opaque"),
            DartFFIDataType::Struct(ident) => f.write_fmt(format_args!(
                "{}{}",
                C_PREFIX,
                DartIdentifier::make_label_for_custom_type(&ident.dart_label)
            )),
            DartFFIDataType::Handle => f.write_str("ffi.Handle"),
            DartFFIDataType::NativeFunction { sub_type } => {
                f.write_fmt(format_args!("ffi.NativeFunction<{}>", sub_type))
            }
            DartFFIDataType::Int8 => f.write_str("ffi.Int8"),
            DartFFIDataType::Int16 => f.write_str("ffi.Int16"),
            DartFFIDataType::Int32 => f.write_str("ffi.Int32"),
            DartFFIDataType::Int64 => f.write_str("ffi.Int64"),
            DartFFIDataType::UIntPtr => f.write_str("ffi.UintPtr"),
            DartFFIDataType::UInt8 => f.write_str("ffi.Uint8"),
            DartFFIDataType::Uint16 => f.write_str("ffi.Uint16"),
            DartFFIDataType::UInt32 => f.write_str("ffi.Uint32"),
            DartFFIDataType::UInt64 => f.write_str("ffi.Uint64"),
            DartFFIDataType::Float => f.write_str("ffi.Float"),
            DartFFIDataType::Double => f.write_str("ffi.Double"),
            DartFFIDataType::Bool => f.write_str("ffi.Bool"),
            DartFFIDataType::Pointer { sub_type } => {
                f.write_fmt(format_args!("ffi.Pointer<{}>", sub_type))
            }
        }
    }
}

#[derive(Debug, Clone)]
enum DartNativeDataType {
    String,
    Int,
    Double,
    CustomClass(String),
    List {
        sub_type: Box<DartNativeDataType>,
    },
    Uri,
    Duration,
    DateTime,
    Map {
        key_type: Box<DartNativeDataType>,
        value_type: Box<DartNativeDataType>,
    },
}

impl Serialize for DartNativeDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<&CType> for DartNativeDataType {
    fn from(c: &CType) -> Self {
        match &c {
            CType::Enum(v) => DartNativeDataType::CustomClass(
                DartIdentifier::make_label_for_custom_type(&v.identifier.label),
            ),
            CType::Struct(v) => DartNativeDataType::CustomClass(
                DartIdentifier::make_label_for_custom_type(&v.identifier.label),
            ),
            CType::Include(_)
            | CType::Function(_)
            | CType::Void
            | CType::VoidStar
            | CType::Define(_, _)
            | CType::UNINITIALIZED => {
                panic!("Cannot create a Dart FFI Type from this ctype: {}", c)
            }
            CType::SignedShort(_)
            | CType::Int8T(_)
            | CType::UnsignedShort(_)
            | CType::UInt8T(_)
            | CType::SignedInteger(_)
            | CType::Int16T(_)
            | CType::UnsignedInteger(_)
            | CType::UInt16T(_)
            | CType::SignedLong(_)
            | CType::Int32T(_)
            | CType::UnsignedLong(_)
            | CType::UInt32T(_)
            | CType::IntPtrT(_)
            | CType::Int64T(_)
            | CType::UIntPtrT(_)
            | CType::UInt64T(_) => DartNativeDataType::Int,
            CType::Float(_) | CType::Double(_) | CType::DoubleDouble(_) => {
                DartNativeDataType::Double
            }
            CType::Char(_) => DartNativeDataType::String,
        }
    }
}

impl Display for DartNativeDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DartNativeDataType::Int => f.write_str("int"),
            DartNativeDataType::String => f.write_str("String"),
            DartNativeDataType::Double => f.write_str("double"),
            DartNativeDataType::CustomClass(c) => f.write_str(c),
            DartNativeDataType::List { sub_type } => {
                f.write_fmt(format_args!("List<{}>", sub_type))
            }
            DartNativeDataType::Uri => f.write_str("Uri"),
            DartNativeDataType::Duration => f.write_str("Duration"),
            DartNativeDataType::DateTime => f.write_str("DateTime"),
            DartNativeDataType::Map {
                key_type,
                value_type,
            } => f.write_fmt(format_args!("Map<{}, {}>", key_type, value_type)),
        }
    }
}

#[derive(Clone)]
enum DartDataType {
    NativeType(DartNativeDataType),
    FFIType(DartFFIDataType),
}

impl Serialize for DartDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            DartDataType::NativeType(data_type) => data_type.serialize(serializer),
            DartDataType::FFIType(data_type) => data_type.serialize(serializer),
        }
    }
}

/// Returns true if `value` is Some(_)
fn is_some(value: Option<&Value>, _: &[Value]) -> tera::Result<bool> {
    match value {
        Some(Value::Null) | None => Ok(false),
        _ => Ok(true),
    }
}

pub fn generate(header: HeaderFile, library_path: &str, library_name: &str) -> String {
    let data = Data::new(
        &header,
        library_path,
        library_name,
        vec![
            "package:ffi/ffi.dart",
            "'dart:ffi' as ffi",
            "'dart:io' show Platform, Directory",
            "'package:path/path.dart' as path",
        ],
    );

    let mut tera = Tera::default();
    tera.register_tester("some", is_some);

    let template_tuples = vec![
        ("generated_header", TEMPLATE_GENERATED_HEADER),
        ("dart_header", TEMPLATE_DART_HEADER),
        ("c_constants", TEMPLATE_C_CONSTANTS),
        ("dart_enums", TEMPLATE_DART_ENUMS),
        ("dart_ffi_structs", TEMPLATE_FFI_STRUCTS),
        ("utilities", TEMPLATE_UTILITY_FUNCTIONS),
    ];

    tera.add_raw_templates(template_tuples.clone()).unwrap();
    let mut s: String = "".to_owned();

    let context = data.make_tera_context();
    for tuple in &template_tuples {
        s = format!("{}{}", s, tera.render(tuple.0, &context).unwrap());
    }

    s
}

const TEMPLATE_GENERATED_HEADER: &'static str = "
// DO NOT MODIFY THIS FILE
// This file contains automatically generated Dart Bindings.
// It was generated via the clang2src project, and ultimately comes from a set of annotated Rust source files
// Any modifications you make to this file will be reverted whenever this file is regenerated
";

const TEMPLATE_DART_HEADER: &str = "
{% if usings|length %}
{% for using in usings %}
 import {{ using }}
{% endfor %}
{% endif %}
";

const TEMPLATE_C_CONSTANTS: &str = "
/// Region: C Constants
const int C_NULL = 0;
const int C_TRUE = 1;
const int C_FALSE = 0;
";

const TEMPLATE_FFI_STRUCTS: &str = "
{% if ffi_structs | length %}
{% for ffi_struct in ffi_structs %}
class {{ ffi_struct.label }} extends {% if ffi_struct.is_opaque %}ffi.Opaque{% else %}ffi.Struct{% endif %} {
    {% for field in ffi_struct.fields %}
    {% if field.comment is some %}{{ field.comment }}{% endif %}
    {% for annotation in field.annotations %}{{ annotation }}
    {% endfor %}
    {% for modifier in field.modifiers %} {{ modifier }} {% endfor %}{{field.kind}} {{field.identifier.dart_label}}
    {% endfor %}
}
{% endfor %}
{% endif %}
";

const TEMPLATE_DART_ENUMS: &str = "
{% block title %}{% endblock %}
{% if enums | length %}
{% for enum in enums %}
{% if enum.dart_comment is some %}{{ enum.dart_comment }}{% endif %}
enum {{ enum.name }} {
    {% for value in enum.values %}
    {% if value.dart_comment is some %}{{ value.dart_comment }}{% endif %}
    {{value.label}}({{ value.value }}),
    {% endfor %}
    ;

    final int value;

    const {{enum.name}}(this.value);
}
{% endfor %}
{% endif %}
";

const TEMPLATE_UTILITY_FUNCTIONS: &str = "
/// Loads the dynamic library using an appropriate extension for the given platform
String getLibraryPath() {
  var libraryPath = path.join(Directory.current.path, '{{meta.library_path}}', '{{meta.library_name}}.so');
  if (Platform.isMacOS || Platform.isIOS) {
    libraryPath = path.join(Directory.current.path, '{{meta.library_path}}', '{{meta.library_name}}.dylib');
  } else if (Platform.isWindows) {
    libraryPath = path.join(Directory.current.path, '{{meta.library_path}}', '{{meta.library_name}}.dll');
  }
  return libraryPath;
}

/// Loads the dynamic library from the given library path
DynamicLibrary loadLibrary(String libraryPath) {
  final dylib = DynamicLibrary.open(libraryPath);
  return dylib;
}

/// Returns a Dart String from a `char*`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For double pointers `char**` use `_getDartStringFromDoublePtr`
String _getDartStringFromPtr(Pointer<Char> charPtr) {
  final asUtf8Ptr = charPtr.cast<Utf8>();
  final asDartString = asUtf8Ptr.toDartString();
  calloc.free(charPtr);
  return asDartString;
}

Pointer<Char> _stringToFFIPointer(String s) {
  return s.toNativeUtf8().cast<Char>();
}

Pointer<Char> _hashMapToFFIPointer(Map<String, String> dict) {
  String val = dict.entries.map((e) => \"${e.key}:${e.value}\").join(';');
  return _stringToFFIPointer(val);
}

/// Returns the Dart equivalent of an empty `char**`
Pointer<Pointer<Char>> _getEmptyStringPointer() {
  return calloc<Pointer<Char>>();
}
";
