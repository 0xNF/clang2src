use core::fmt::Display;
use core::panic;
use serde::ser::{SerializeMap, SerializeSeq};
use serde_json::Value;
use std::borrow::BorrowMut;
use std::fmt::Formatter;
use tera::{Context, Tera};

use serde::{Serialize, Serializer};

use crate::lexer::{
    CEnum, CFunction, CStruct, CType, CVariableDeclaration, CVariableType, HeaderFile,
};
use crate::meta::{MetaValue, META_TOKEN};

const C_PREFIX: &str = "C_";
const C_FUNCTION_PREFIX: &str = "ffi_";
const C_PARAMETER_NAME: &str = "c";
const DART_FACTORY_KEYWORD: &str = "factory";

const DART_KEYWORDS: &'static [&'static str] = &[
    "abstract",
    "else",
    "import",
    "show",
    "as",
    "enum",
    "static",
    "assert",
    "export",
    "interface",
    "super",
    "async",
    "extends",
    "is",
    "switch",
    "await",
    "extension",
    "late",
    "sync",
    "break",
    "external",
    "library",
    "this",
    "case",
    "factory",
    "mixin",
    "throw",
    "catch",
    "false",
    "new",
    "true",
    "class",
    "final",
    "null",
    "try",
    "const",
    "finally",
    "on",
    "typedef",
    "continue",
    "for",
    "operator",
    "var",
    "covariant",
    "Function",
    "part",
    "void",
    "default",
    "get",
    "required",
    "while",
    "deferred",
    "hide",
    "rethrow",
    "with",
    "do",
    "if",
    "return",
    "yield",
    "dynamic",
    "implements",
    "set",
];

#[derive(Serialize)]
struct Data<'a> {
    library_path: &'a str,
    library_name: &'a str,
    usings: Vec<&'a str>,
    enums: Vec<DartEnum>,
    ffi_structs: Vec<DartFFIStruct>,
    ffi_functions: Vec<DartFunction>,
    constants: Vec<DartVariable>,
    native_free_functions: Vec<DartFunction>,
    dart_classes: Vec<DartClass>,
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

        let constants: Vec<DartVariable> = header
            .defines
            .iter()
            .map(|v| DartVariable::from(v, false))
            .collect();

        let ffi_functions: Vec<DartFunction> = header
            .functions
            .iter()
            .map(|f| DartFunction::from(f, true))
            .collect();

        let mut dart_classes: Vec<DartClass> = ffi_structs.iter().map(DartClass::from).collect();

        let native_free_functions: Vec<DartFunction> = header
            .functions
            .iter()
            .filter(|f| match &f.meta {
                None => true,
                Some(m) => !m.for_struct,
            })
            .map(|f| DartFunction::from(f, false))
            .collect();

        /* Attach methods to classes */
        for f in header.functions.iter().filter(|m| match &m.meta {
            Some(meta) => meta.for_struct,
            None => false,
        }) {
            let idx_of_under = f.label.find('_').unwrap();
            let struct_name = &f.label[..idx_of_under];
            let func_name = &f.label[idx_of_under..];

            let struct_name = DartIdentifier::make_label_for_custom_type(struct_name);
            let func_name = DartIdentifier::new(func_name, None);

            let on_class = dart_classes
                .iter_mut()
                .filter(|dc| dc.identifier.dart_label == struct_name)
                .next()
                .unwrap();

            /* Find Main Constructor */
            if f.meta.as_ref().map_or(false, |m| m.is_constructor) {
                let meta = f.meta.to_owned().unwrap();
                let df = DartFunction {
                    is_private: false,
                    on_class: Some(on_class.identifier.to_owned()),
                    identifier: DartIdentifier::new_from_raw(&format!(
                        "{}Create",
                        func_name.dart_label
                    )),
                    c_function_name: Some(f.label.to_owned()),
                    dart_comment: f.comment.to_owned().map(DartComment::from),
                    is_void: false,
                    throws: meta.throws,
                    output_requires_pointer: true,
                    is_async: meta.is_async,
                    parameters: f
                        .parameters
                        .iter()
                        .filter(|param| match &param.meta {
                            Some(param_meta) => {
                                !(param_meta.is_output || param_meta.is_error || param_meta.is_this)
                            }
                            None => true,
                        })
                        .map(|c| DartParameter::from(c, false))
                        .collect(),
                    meta: Some(meta.to_owned()),
                    return_type: DartDataType::NativeType(DartNativeDataType::CustomClass(
                        on_class.identifier.dart_label.to_owned(),
                    )),
                    ffi_return_type: None,
                    is_return_struct: false,
                    requires_ffi_function_pointers: false,
                    annotations: vec![],
                    modifiers: vec![DART_FACTORY_KEYWORD.to_owned()],
                    is_factory: true,
                    body: None,
                };
                on_class.functions.push(df);
            } else {
                /* Add for_struct function outside of Constructor */
                let fmeta = f.meta.to_owned().unwrap();
                if !fmeta.is_destructor {
                    // FYI(nf, 04/21/23): dont worry about destructors. They are handled with _finalizer attachements
                    let mut is_return_struct: bool = false;
                    let df = DartFunction {
                        is_private: false,
                        is_async: fmeta.is_async,
                        is_factory: false,
                        output_requires_pointer: true,
                        requires_ffi_function_pointers: false,
                        throws: fmeta.throws,
                        is_void: fmeta.is_void,
                        c_function_name: Some(f.label.to_owned()),
                        dart_comment: f.comment.to_owned().map(DartComment::from),
                        on_class: Some(on_class.identifier.to_owned()),
                        identifier: func_name,
                        return_type: if fmeta.is_void {
                            DartDataType::NativeType(DartNativeDataType::Void)
                        } else {
                            /* Extract the designated `Output` field if it exists */
                            if let Some(param) = f.parameters.iter().find(|p| match &p.meta {
                                Some(param_meta) => param_meta.is_output,
                                None => false,
                            }) {
                                is_return_struct = param.variable_type.is_struct;
                                DartDataType::from_meta(
                                    &param.label,
                                    &param.variable_type,
                                    &param.meta,
                                    false,
                                )
                            } else {
                                /* If no `output` field exists, and the return value isn't `void`, then just return whatever the face-value return is, transformed for Dart */
                                DartDataType::from(&*f.return_type)
                            }
                        },
                        is_return_struct,
                        modifiers: vec![],
                        annotations: vec![],
                        parameters: f
                            .parameters
                            .iter()
                            .filter(|p| match &p.meta {
                                Some(param_meta) => {
                                    !(param_meta.is_error
                                        || param_meta.is_output
                                        || param_meta.is_this)
                                }
                                None => true,
                            })
                            .map(|p| DartParameter::from(p, false))
                            .collect(),
                        meta: f.meta.to_owned(),
                        ffi_return_type: Some(DartFFIDataType::from(&*f.return_type)),
                        body: None,
                    };
                    on_class.functions.push(df);
                }
            }
        }

        Data {
            library_path,
            library_name,
            usings,
            enums,
            ffi_structs,
            constants,
            ffi_functions,
            native_free_functions,
            dart_classes,
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
        context.insert("ffi_functions", &self.ffi_functions);
        context.insert("constants", &self.constants);
        context.insert("dart_classes", &self.dart_classes);
        context.insert("dart_native_free_functions", &self.native_free_functions);

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

impl DartComment {
    fn from_raw(s: &str) -> Self {
        DartComment {
            inner: s.to_owned(),
        }
    }
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
    /// Appended to the end of an identifier if the identifier conflicts with anything else in scope
    const KEYWORD_CONFLICT_APPENDER: &'static str = "_";

    fn append_keyword_conflict(label: &str, in_scope: Vec<&str>) -> String {
        let mut new_label: String = label.to_owned();
        while in_scope.contains(&new_label.as_str()) {
            new_label.push_str(DartIdentifier::KEYWORD_CONFLICT_APPENDER);
        }
        new_label
    }

    fn make_label_for_constant(label: &str) -> String {
        label.to_uppercase()
    }

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

        DartIdentifier {
            dart_label: Self::transform_label(label),
            dart_comment,
        }
    }

    fn new_for_constant(label: &str, comment: Option<String>) -> Self {
        DartIdentifier {
            dart_label: DartIdentifier::append_keyword_conflict(
                DartIdentifier::make_label_for_constant(label).as_str(),
                DART_KEYWORDS.to_vec(),
            ),
            dart_comment: comment.map(DartComment::from),
        }
    }

    fn new_for_custom_type(label: &str, comment: Option<String>, for_ffi: bool) -> Self {
        let label = if for_ffi {
            format!("{}{}", C_PREFIX, Self::make_label_for_custom_type(label))
        } else {
            Self::make_label_for_custom_type(label)
        };

        DartIdentifier {
            dart_label: DartIdentifier::append_keyword_conflict(
                label.as_str(),
                DART_KEYWORDS.to_vec(),
            ),
            dart_comment: comment.map(DartComment::from),
        }
    }

    fn new_from_raw(label: &str) -> Self {
        DartIdentifier {
            dart_label: DartIdentifier::append_keyword_conflict(label, DART_KEYWORDS.to_vec()),
            dart_comment: None,
        }
    }
}

#[derive(Clone, Serialize, Debug)]
struct DartEnum {
    /// Whether this class should be visible outside the generated pacage
    is_private: bool,
    name: String,
    values: Vec<DartEnumOption>,
    dart_comment: Option<DartComment>,
    meta: Option<MetaValue>,
}

impl From<&CEnum> for DartEnum {
    fn from(c: &CEnum) -> Self {
        let enum_name = DartIdentifier::make_label_for_custom_type(&c.identifier.label);
        let dart_comment = c.comment.to_owned().map(DartComment::from);

        let values: Vec<DartEnumOption> = c
            .declarations
            .iter()
            .enumerate()
            .map(|(idx, ident)| DartEnumOption::new(&ident.label, ident.comment.to_owned(), idx))
            .collect();

        DartEnum {
            is_private: false,
            name: enum_name,
            dart_comment,
            values,
            meta: c.meta.as_ref().map(|meta| meta.clone()),
        }
    }
}

#[derive(Clone, Serialize, Debug)]
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
#[derive(Serialize, Clone, Debug)]
struct DartField {
    /// Whether this field should be visible outside the generated pacage
    is_private: bool,

    /// Whether this field can be marked `field?` for non-nullability
    is_nullable: bool,

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

    meta: Option<MetaValue>,

    reads_capacity_from: Option<String>,
    reads_length_from: Option<String>,

    /// Optionally in-line defined text for an immediate assignment
    assign_statement: Option<String>,

    as_primitive_kind: DartDataType,
}

impl DartField {
    /// Takes a DartField structured for use on an FFIStruct and returns a new DartField
    /// fit for use on an externally facing Dart Class
    fn into_class_field(&self) -> Self {
        DartField {
            is_nullable: self.meta.as_ref().map_or(false, |f| f.is_nullable),
            is_private: self.is_private,
            identifier: DartIdentifier::new_from_raw(&self.identifier.dart_label),
            comment: self.comment.to_owned(),
            annotations: vec![],
            modifiers: vec!["final".to_owned()],
            kind: self.kind.to_upper_type(self.meta.as_ref()),
            meta: self.meta.to_owned(),
            assign_statement: None,
            reads_length_from: None,
            reads_capacity_from: None,
            as_primitive_kind: self.as_primitive_kind.to_owned(),
        }
    }
}

/// A parameter, as in an argument to a function
#[derive(Serialize, Clone, Debug)]
struct DartParameter {
    /// Whether to mark the field type as nullable
    is_nullable: bool,
    /// whether to mark this item as 'required'
    is_required: bool,
    /// Fully qualified Dart Native type of this item,
    kind: DartDataType,

    /// If true, dont drop this item with the free function after use
    is_persistent: bool,

    /// Whether this item needs to be wrapped in an additional Pointer<> container
    requires_pointer: bool,

    /// The Dart FFI type that backs this type
    ffi_kind: DartFFIDataType,

    /// Dart-ified label for this item
    identifier: DartIdentifier,

    /// If set, the default value for constructions of the form {named_value = some_value}
    default_value: Option<DartValue>,

    /// When handed in as a function parameter, the transform to apply. Normally used for enums, which
    /// need to be handed across FFI boundaries as `enumItem.value`
    as_ffi_value: Option<String>,

    /// The DartFFI type that this type can decays into
    /// e.g.,
    /// class, string, int, void, map => self
    /// uri => string,
    /// duration, datetime, enum => int,
    as_primitive_kind: DartDataType,
}

impl DartParameter {
    fn from(c: &CVariableDeclaration, as_ffi: bool) -> Self {
        let identifier = DartIdentifier::new_from_raw(&c.label);
        let kind = DartDataType::from_meta(&c.label, &c.variable_type, &c.meta, as_ffi);
        DartParameter {
            is_nullable: c.meta.as_ref().map_or(false, |f| f.is_nullable),
            is_required: false,
            is_persistent: c.variable_type.is_struct,
            ffi_kind: DartFFIDataType::from(c),
            as_ffi_value: Some(if matches!(c.variable_type.kind, CType::Enum(_)) {
                format!("{}.getValueAsInt", identifier.dart_label)
            } else {
                identifier.clone().dart_label
            }),
            requires_pointer: kind.requires_pointer(),
            as_primitive_kind: kind.to_primitive(),
            identifier,
            kind,
            default_value: None,
        }
    }
}

#[derive(Clone, Debug)]
struct DartValue {
    value: String,
}

impl DartValue {
    fn new(value: &str) -> Self {
        DartValue {
            value: value.to_owned(),
        }
    }
}

impl Serialize for DartValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.value)
    }
}

impl From<&CType> for DartValue {
    fn from(c: &CType) -> Self {
        match &c {
            CType::Enum(v) => DartValue::new(&format!(
                "{}()",
                &DartIdentifier::make_label_for_custom_type(v.identifier.label.as_str())
            )),
            CType::Struct(v) => DartValue::new(&format!(
                "{}()",
                &DartIdentifier::make_label_for_custom_type(v.identifier.label.as_str())
            )),

            CType::Include(_) | CType::UNINITIALIZED => {
                panic!("Cannot make DartValue for CType: {}", c)
            }
            CType::SignedShort(v) => DartValue::new(&v.to_string()),
            CType::UnsignedShort(v) => DartValue::new(&v.to_string()),
            CType::SignedInteger(v) => DartValue::new(&v.to_string()),
            CType::UnsignedInteger(v) => DartValue::new(&v.to_string()),
            CType::SignedLong(v) => DartValue::new(&v.to_string()),
            CType::UnsignedLong(v) => DartValue::new(&v.to_string()),
            CType::Int64T(v) => DartValue::new(&v.to_string()),
            CType::Float(v) => DartValue::new(&v.to_string()),
            CType::Double(v) => DartValue::new(&v.to_string()),
            CType::DoubleDouble(v) => DartValue::new(&v.to_string()),
            CType::Char(v) => DartValue::new(&format!("'{}'", v.to_string())),
            CType::Function(v) => DartValue::new(v.label.as_str()),
            CType::Define(label, typeval) => DartValue::from(&**typeval),
            CType::IntPtrT(v) => DartValue::new(&v.to_string()),
            CType::UIntPtrT(v) => DartValue::new(&v.to_string()),
            CType::Int8T(v) => DartValue::new(&v.to_string()),
            CType::Int16T(v) => DartValue::new(&v.to_string()),
            CType::Int32T(v) => DartValue::new(&v.to_string()),
            CType::UInt8T(v) => DartValue::new(&v.to_string()),
            CType::UInt16T(v) => DartValue::new(&v.to_string()),
            CType::UInt32T(v) => DartValue::new(&v.to_string()),
            CType::UInt64T(v) => DartValue::new(&v.to_string()),
            CType::VoidStar => DartValue::new("null"),
            CType::Void => DartValue::new("void"),
        }
    }
}

#[derive(Debug, Serialize)]
struct DartVariable {
    identifier: DartIdentifier,
    c_comment: Option<String>,
    dart_comment: Option<DartComment>,
    value: Option<DartValue>,
    data_type: DartDataType,
    pointer_count: u8,
    is_last: bool,
    meta: MetaValue,
}

impl DartVariable {
    fn from(c: &CVariableDeclaration, as_ffi: bool) -> Self {
        let identifier = DartIdentifier::new_for_constant(&c.label, None);

        let ddt = if as_ffi {
            DartDataType::FFIType(DartFFIDataType::from(&c.variable_type))
        } else {
            DartDataType::NativeType(DartNativeDataType::from(c))
        };

        let value = DartValue::from(&c.variable_type.kind);

        DartVariable {
            identifier,
            dart_comment: c.comment.clone().map(DartComment::from),
            c_comment: c.comment.clone(),
            data_type: ddt,
            pointer_count: c.variable_type.pointer_count,
            is_last: false,
            meta: MetaValue::new(),
            value: Some(value),
        }
    }
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

#[derive(Serialize, Debug)]
struct DartConstructor {
    is_private: bool,
    identifier: DartIdentifier,
    parameters: Vec<String>,
    body: Option<String>,
}

#[derive(Serialize, Debug)]
struct DartClass {
    /// Whether this class should be visible outside the generated pacage
    is_private: bool,
    identifier: DartIdentifier,
    backing_ffi_struct: Option<DartIdentifier>,
    fields: Vec<DartField>,
    functions: Vec<DartFunction>,
    constructors: Vec<DartConstructor>,
    /// Extends Classes
    extends: Vec<String>,
    /// Implements interfaces
    implements: Vec<String>,
    meta: Option<MetaValue>,
}

impl From<&DartFFIStruct> for DartClass {
    fn from(f: &DartFFIStruct) -> Self {
        let backing_struct_identifier = DartIdentifier::new_from_raw(f.label.as_str());
        let class_identifier = DartIdentifier::new_from_raw(f.label.trim_start_matches(C_PREFIX));

        let extends: Vec<String> = vec![];
        let mut implements: Vec<String> = vec![];
        let mut fields: Vec<DartField> = vec![];
        let mut constructors: Vec<DartConstructor> = vec![];

        /* Add actual class fieldss */
        fields.append(
            &mut f
                .fields
                .iter()
                .map(|df| df.into_class_field())
                .collect::<Vec<DartField>>(),
        );

        /* parse out aspects that rely on other fields, such as CapacityFor and LengthFor */

        for field in fields.clone().iter() {
            if let Some(field_meta) = &field.meta {
                if let Some(length_for_label) = &field_meta.length_for {
                    if let Some(length_item) = fields.iter_mut().find(|field| {
                        field.identifier.dart_label
                            == DartIdentifier::new_from_raw(&length_for_label).dart_label
                    }) {
                        length_item.reads_length_from =
                            Some(field.identifier.dart_label.to_owned());
                    }
                } else if let Some(capacity_for_label) = &field_meta.capacity_for {
                    if let Some(capacity_item) = fields.iter_mut().find(|field| {
                        field.identifier.dart_label
                            == DartIdentifier::new_from_raw(&capacity_for_label).dart_label
                    }) {
                        capacity_item.reads_capacity_from =
                            Some(field.identifier.dart_label.to_owned());
                    }
                }
            }
        }

        if f.is_opaque {
            /* _fromCPtr */
            constructors.push(DartConstructor {
                is_private: true,
                parameters: vec!["_selfPtr".to_owned()],
                identifier: DartIdentifier::new_from_raw("fromCPtr"),
                body: Some(
                    "
                _finalizer.attach(this, _selfPtr.cast(), detach: this);
                "
                    .to_owned(),
                ),
            });
        } else {
            /* _fromFields */
            constructors.push(DartConstructor {
                is_private: true,
                identifier: DartIdentifier::new_from_raw("fromFields"),
                parameters: fields
                    .iter()
                    .map(|f| f.identifier.dart_label.to_owned())
                    .collect(),
                body: None,
            });
        }

        let mut functions: Vec<DartFunction> = vec![
            /* fromCPointer */
            DartFunction {
                is_private: false,
                modifiers: vec![DART_FACTORY_KEYWORD.to_owned()],
                annotations: vec![],
                c_function_name: None,
                dart_comment: Some(DartComment::from_raw(&format!(
                    "/// Creates an instance of this class from a Pointer<{}>",
                    f.label
                ))),
                identifier: DartIdentifier::new_from_raw("_fromCPointer"),
                ffi_return_type: None,
                is_async: false,
                is_return_struct: false,
                is_void: false,
                output_requires_pointer: false,
                requires_ffi_function_pointers: false,
                throws: false,
                on_class: Some(class_identifier.to_owned()),
                is_factory: true,
                return_type: DartDataType::NativeType(DartNativeDataType::Void),
                meta: None,
                parameters: {
                    let ffi_kind = DartFFIDataType::Pointer {
                        sub_type: Box::new(DartFFIDataType::Void),
                    };
                    let dkind = DartDataType::FFIType(ffi_kind.to_owned());
                    vec![DartParameter {
                        is_nullable: false,
                        is_required: false,
                        is_persistent: false,
                        default_value: None,
                        requires_pointer: false,
                        identifier: DartIdentifier::new_from_raw("voidPtr"),
                        as_ffi_value: None,
                        as_primitive_kind: dkind.to_primitive(),
                        kind: dkind.clone(),
                        ffi_kind,
                    }]
                },
                body: Some(
                    if f.is_opaque {
                        format!(
                            "
                    return {}._fromCPtr(voidPtr.cast());
                        ",
                            class_identifier.dart_label
                        )
                    } else {
                        format!(
                            "
                    ffi.Pointer<{}> ptr = voidPtr.cast();
                    final st = ptr.ref;
                    return {}._fromCStruct(st);",
                            backing_struct_identifier.dart_label, class_identifier.dart_label,
                        )
                    }
                    .to_owned(),
                ),
            },
            /* fromCPointerPointer */
            DartFunction {
                is_private: false,
                modifiers: vec![DART_FACTORY_KEYWORD.to_owned()],
                annotations: vec![],
                c_function_name: None,
                dart_comment: Some(DartComment::from_raw(&format!(
                    "/// Creates an instance of this class from a Pointer<Pointer<{}>>",
                    f.label
                ))),
                identifier: DartIdentifier::new_from_raw("_fromCPointerPointer"),
                ffi_return_type: None,
                is_async: false,
                is_return_struct: false,
                is_void: false,
                output_requires_pointer: false,
                requires_ffi_function_pointers: false,
                throws: false,
                on_class: Some(class_identifier.to_owned()),
                is_factory: true,
                return_type: DartDataType::NativeType(DartNativeDataType::Void),
                meta: None,
                parameters: {
                    let ffi_kind = DartFFIDataType::Pointer {
                        sub_type: Box::new(DartFFIDataType::Pointer {
                            sub_type: Box::new(DartFFIDataType::Void),
                        }),
                    };
                    let dkind = DartDataType::FFIType(ffi_kind.to_owned());
                    vec![DartParameter {
                        is_nullable: false,
                        is_required: false,
                        is_persistent: false,
                        default_value: None,
                        requires_pointer: false,
                        identifier: DartIdentifier::new_from_raw("voidPtr"),
                        as_ffi_value: None,
                        as_primitive_kind: dkind.to_primitive(),
                        kind: dkind.clone(),
                        ffi_kind,
                    }]
                },
                body: Some(
                    if f.is_opaque {
                        format!(
                            "
                return {}._fromCPointer(voidPtr.value);
                    ",
                            class_identifier.dart_label
                        )
                    } else {
                        format!(
                            "
                return {}._fromCPointer(voidPtr.value);
                ",
                            class_identifier.dart_label,
                        )
                    }
                    .to_owned(),
                ),
            },
        ];

        /* fromCStruct */
        if !f.is_opaque {
            functions.push(DartFunction {
                is_private: false,
                modifiers: vec![DART_FACTORY_KEYWORD.to_owned()],
                c_function_name: None,
                dart_comment: Some(DartComment::from_raw(
                    "/// Creates an instance of this class from a struct reference",
                )),
                identifier: DartIdentifier::new_from_raw("_fromCStruct"),
                annotations: vec![],
                ffi_return_type: None,
                is_async: false,
                is_return_struct: false,
                is_void: false,
                output_requires_pointer: false,
                requires_ffi_function_pointers: false,
                throws: false,
                on_class: Some(class_identifier.to_owned()),
                is_factory: true,
                return_type: DartDataType::NativeType(DartNativeDataType::Void),
                meta: None,
                parameters: {
                    let ffi_kind = DartFFIDataType::Struct(backing_struct_identifier.to_owned());
                    let ddt = DartDataType::FFIType(ffi_kind.to_owned());
                    vec![DartParameter {
                        is_required: false,
                        is_nullable: false,
                        is_persistent: false,
                        as_ffi_value: None,
                        default_value: None,
                        requires_pointer: false,
                        as_primitive_kind: ddt.to_owned(),
                        kind: ddt,
                        identifier: DartIdentifier::new_from_raw(C_PARAMETER_NAME),
                        ffi_kind,
                    }]
                },
                body: None,
            })
        }

        if let Some(m) = &f.meta {
            /* Make Self Pointer if necessary */
            if m.is_persistent {
                /* Make Self Ptr Field */
                {
                    let dkind = DartDataType::FFIType(DartFFIDataType::Pointer {
                        sub_type: Box::new(DartFFIDataType::Struct(DartIdentifier::new_from_raw(
                            f.label.as_str(),
                        ))),
                    });

                    let _self_ptr: DartField = DartField {
                        is_nullable: m.is_nullable,
                        is_private: true,
                        modifiers: vec![],
                        annotations: vec![],
                        as_primitive_kind: dkind.to_owned(),
                        comment: f.comment.to_owned(),
                        identifier: DartIdentifier::new_from_raw("selfPtr"),
                        meta: None,
                        kind: dkind,
                        assign_statement: None,
                        reads_capacity_from: None,
                        reads_length_from: None,
                    };

                    fields.push(_self_ptr);
                }

                /* Make _IWithPtr function */
                implements.push("_IWithPtr".to_owned());

                functions.push(DartFunction {
                    is_private: false,
                    on_class: Some(class_identifier.to_owned()),
                    identifier: DartIdentifier::new_from_raw("getPointer"),
                    c_function_name: None,
                    dart_comment: None,
                    is_void: false,
                    throws: false,
                    output_requires_pointer: false,
                    is_async: false,
                    parameters: vec![],
                    meta: None,
                    return_type: DartDataType::FFIType(DartFFIDataType::Pointer {
                        sub_type: Box::new(DartFFIDataType::Void),
                    }),
                    ffi_return_type: None,
                    is_return_struct: false,
                    requires_ffi_function_pointers: false,
                    annotations: vec!["@override".to_owned()],
                    modifiers: vec![],
                    is_factory: false,
                    body: Some("return _selfPtr.cast();".to_owned()),
                });

                /* make Finalizable for free'ing  */
                implements.push("ffi.Finalizable".to_owned());
                fields.push(DartField {
                    is_nullable: false,
                    is_private: true,
                    reads_capacity_from: None,
                    reads_length_from: None,
                    identifier: DartIdentifier::new_from_raw("finalizer"),
                    comment: Some(DartComment::from_raw("/// Pointer to the backing `free` function which disposes the backing pointer when this dart object is GC'd")),
                    annotations: vec![],
                    modifiers: vec!["static".to_owned(), "final".to_owned()],
                    kind: DartDataType::FFIType(DartFFIDataType::NativeFinalizer),
                    as_primitive_kind: DartDataType::FFIType(DartFFIDataType::NativeFinalizer),
                    assign_statement: Some(format!("ffi.NativeFinalizer({}{}_freePtr.cast())", C_FUNCTION_PREFIX, backing_struct_identifier.dart_label.trim_start_matches(C_PREFIX))),
                    meta: Some({
                        let mut meta_value: MetaValue = MetaValue::new();
                        meta_value.is_static = true;
                        meta_value
                    }),
                })
            }
        }

        DartClass {
            is_private: false,
            identifier: class_identifier,
            backing_ffi_struct: Some(backing_struct_identifier),
            constructors,
            extends,
            implements,
            fields,
            /// appended to at a later stage, after all the functions have been parsed
            functions,
            meta: f.meta.to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
struct DartFunction {
    /// Whether this function should be visible outside the generated pacage
    is_private: bool,
    /// Whether this function should be attached to a class
    on_class: Option<DartIdentifier>,
    /// Dart friendly label for this function
    identifier: DartIdentifier,
    /// Underlying C function name for this function (only for direct-from-c funtions)
    c_function_name: Option<String>,
    /// Dart friendly comment for this function (stripped of #meta)
    dart_comment: Option<DartComment>,
    /// Whether this function returns `void`
    is_void: bool,

    /// Whether this function `throws` via the `ErrPtr` string
    throws: bool,

    /// Whether the return vaue requires pointer manipulation
    output_requires_pointer: bool,

    /// Whether this function is of the form `Future<T> xyz() async`
    is_async: bool,

    /// Inputs to this function
    parameters: Vec<DartParameter>,

    /// #meta values for determining serialization stuff
    meta: Option<MetaValue>,

    /// Dart friendly return value. For FFI functions, may return Pointer<xyz> or C_{Element_Name}
    return_type: DartDataType,

    ffi_return_type: Option<DartFFIDataType>,

    /// Whether this function returns a C_{Struct}
    is_return_struct: bool,

    /// Whether this function needs additional helper functions in the form of `_{func}Ptr = _lookup...` and `_{func}` = _{func}Ptr.asFunction...`
    requires_ffi_function_pointers: bool,

    /// e.g., '@override'
    annotations: Vec<String>,

    /// e.g., 'factory'
    modifiers: Vec<String>,

    is_factory: bool,

    /// Optional concrete implenetation of this method
    body: Option<String>,
}

impl DartFunction {
    fn from(c: &CFunction, as_ffi: bool) -> Self {
        let mut return_type = DartDataType::from(&*c.return_type);

        let mut throws: bool = false;

        let mut requires_pointer: bool = false;

        DartFunction {
            is_private: false,
            on_class: None,
            is_factory: false,
            identifier: {
                let label = c.label.as_str();
                if as_ffi {
                    DartIdentifier::new_from_raw(&format!("{}{}", C_FUNCTION_PREFIX, label))
                } else {
                    DartIdentifier::new_from_raw(label)
                }
            },
            c_function_name: Some(c.label.to_owned()),
            dart_comment: c.comment.to_owned().map(DartComment::from),
            is_void: matches!(c.return_type.kind, CType::Void),
            is_async: c.meta.as_ref().map_or(false, |m| m.is_async),
            modifiers: vec![],
            annotations: vec![],
            body: None,
            parameters: {
                if !as_ffi {
                    throws = true;
                    // FYI(nf, 04/17/23): non-ffi items should be stripped of their Output and Error params
                    let params: Vec<DartParameter> = c
                        .parameters
                        .iter()
                        .filter(|p| match &p.meta {
                            Some(m) => !(m.is_output || m.is_error),
                            None => true,
                        })
                        .map(|p| DartParameter::from(p, false))
                        .collect();

                    if c.parameters
                        .iter()
                        .filter(|p| match &p.meta {
                            Some(m) => m.is_error,
                            None => false,
                        })
                        .map(|p: &CVariableDeclaration| DartNativeDataType::from(p))
                        .collect::<Vec<DartNativeDataType>>()
                        .is_empty()
                    {
                        throws = false;
                    }

                    // Check output
                    if let Some(p) = c
                        .parameters
                        .iter()
                        .filter(|p| match &p.meta {
                            Some(m) => m.is_output,
                            None => false,
                        })
                        .map(|p: &CVariableDeclaration| DartNativeDataType::from(p))
                        .collect::<Vec<DartNativeDataType>>()
                        .first()
                    {
                        return_type = DartDataType::NativeType(p.to_owned());
                        requires_pointer = return_type.requires_pointer();
                        // return_type_struct = param.variable_type.is_struct;
                    }

                    /// Check `this`
                    let this: Option<&DartParameter> = c
                        .parameters
                        .iter()
                        .filter(|p| match &p.meta {
                            Some(m) => m.is_this,
                            None => false,
                        })
                        .map(|p| DartParameter::from(p, false))
                        .collect::<Vec<DartParameter>>()
                        .first();

                    params
                } else {
                    c.parameters
                        .iter()
                        .map(|p| DartParameter::from(p, as_ffi))
                        .collect()
                }
            },
            meta: c.meta.as_ref().map(|meta| meta.clone()),
            ffi_return_type: {
                let mut pointer_count = c.return_type.pointer_count;
                let mut ffi_type = DartFFIDataType::from(&*c.return_type);
                while pointer_count > 0 {
                    ffi_type = DartFFIDataType::Pointer {
                        sub_type: Box::new(ffi_type),
                    };
                    pointer_count -= 1;
                }
                Some(ffi_type)
            },
            throws,
            return_type,
            output_requires_pointer: requires_pointer,
            is_return_struct: c.return_type.is_struct,
            requires_ffi_function_pointers: true, // always true here because we are From'ing from a CFunction
        }
    }
}

/// The FFI backing structure for a Dart class
#[derive(Serialize)]
struct DartFFIStruct {
    /// Whether this class should be visible outside the generated pacage
    is_private: bool,

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

    /// Extends Classes
    extends: Vec<String>,

    /// Implements interfaces
    implements: Vec<String>,

    meta: Option<MetaValue>,

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
                let kind = if decl.variable_type.is_struct {
                    DartDataType::FFIType(DartFFIDataType::Pointer {
                        sub_type: Box::new(ffi_kind.to_owned()),
                    })
                } else if matches!(decl.variable_type.kind, CType::Char(_))
                    && decl.variable_type.pointer_count > 0
                {
                    DartDataType::FFIType(DartFFIDataType::from(decl))
                } else {
                    DartDataType::FFIType(ffi_kind.to_owned())
                };

                DartField {
                    is_nullable: c.meta.as_ref().map_or(false, |f| f.is_nullable),
                    is_private: false,
                    identifier: DartIdentifier::new(&decl.label, None),
                    comment: decl.comment.to_owned().map(DartComment::from),
                    annotations: vec![ffi_kind.get_dart_annotation_string()],
                    modifiers: vec!["external".to_owned()],
                    as_primitive_kind: kind.for_struct(),
                    assign_statement: None,
                    reads_capacity_from: None,
                    reads_length_from: None,
                    meta: decl.meta.to_owned(),
                    kind,
                }
            })
            .collect();
        DartFFIStruct {
            is_private: false,
            is_opaque: c.declarations.len() == 0,
            label: format!(
                "{}{}",
                C_PREFIX,
                DartIdentifier::make_label_for_custom_type(&c.identifier.label)
            ),
            comment,
            fields,
            meta: c.meta.to_owned(),
            extends: vec![if c.declarations.len() == 0 {
                "ffi.Opaque".to_owned()
            } else {
                "ffi.Struct".to_owned()
            }],
            implements: vec![],
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
    NativeFinalizer,
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

impl Into<DartNativeDataType> for &DartFFIDataType {
    fn into(self) -> DartNativeDataType {
        (self.to_owned()).into()
    }
}

impl Into<DartNativeDataType> for DartFFIDataType {
    fn into(self) -> DartNativeDataType {
        match &self {
            DartFFIDataType::Pointer { sub_type } => (*sub_type.to_owned()).into(),
            DartFFIDataType::Opaque(ident) => DartNativeDataType::CustomEnum(
                DartIdentifier::make_label_for_custom_type(&ident.dart_label),
            ),
            DartFFIDataType::Struct(ident) => {
                DartNativeDataType::CustomClass(DartIdentifier::make_label_for_custom_type(
                    &ident.dart_label.trim_start_matches(C_PREFIX),
                ))
            }
            DartFFIDataType::NativeType
            | DartFFIDataType::Handle
            | DartFFIDataType::NativeFunction { sub_type: _ }
            | DartFFIDataType::NativeFinalizer => panic!(
                "Cannot convert this FFI Type into a Dart: Native Type: {}",
                self
            ),
            DartFFIDataType::Char => DartNativeDataType::String,
            DartFFIDataType::Void => DartNativeDataType::Void,
            DartFFIDataType::Int8
            | DartFFIDataType::Int16
            | DartFFIDataType::Int32
            | DartFFIDataType::Int64
            | DartFFIDataType::UIntPtr
            | DartFFIDataType::UInt8
            | DartFFIDataType::Uint16
            | DartFFIDataType::UInt32
            | DartFFIDataType::UInt64 => DartNativeDataType::Int,
            DartFFIDataType::Float => DartNativeDataType::Double,
            DartFFIDataType::Double => DartNativeDataType::Double,
            DartFFIDataType::Bool => DartNativeDataType::Bool,
        }
    }
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
            CType::Include(_)
            | CType::VoidStar
            | CType::Define(_, _)
            | CType::Function(_)
            | CType::UNINITIALIZED => {
                panic!("Cannot create a Dart FFI Type from this ctype: {}", c)
            }
            CType::Enum(e) => DartFFIDataType::Int32,
            CType::Struct(c) => DartFFIDataType::Struct(DartIdentifier::new_for_custom_type(
                &c.identifier.label,
                None,
                true,
            )),
            CType::Void => DartFFIDataType::Void,
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
        let ffi_type = DartFFIDataType::from(&c.kind);
        ffi_type
    }
}

impl From<&CVariableDeclaration> for DartFFIDataType {
    fn from(c: &CVariableDeclaration) -> Self {
        let mut pointer_count = c.variable_type.pointer_count;
        let mut ffi_type = DartFFIDataType::from(&c.variable_type);
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
            | DartFFIDataType::Char
            | DartFFIDataType::NativeFinalizer
            | DartFFIDataType::Void => "".to_owned(),
            DartFFIDataType::Int64
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
                format!("@{}()", self.to_string())
            }
        }
    }

    fn requires_pointer(&self) -> bool {
        match &self {
            DartFFIDataType::NativeType
            | DartFFIDataType::Handle
            | DartFFIDataType::NativeFinalizer
            | DartFFIDataType::NativeFunction { sub_type: _ } => {
                panic!("cannot check pointerness of this type: {}", &self)
            }
            DartFFIDataType::Pointer { sub_type: _ }
            | DartFFIDataType::Void
            | DartFFIDataType::Char
            | DartFFIDataType::Opaque(_)
            | DartFFIDataType::Struct(_) => true,
            DartFFIDataType::Int8
            | DartFFIDataType::Int16
            | DartFFIDataType::Int32
            | DartFFIDataType::Int64
            | DartFFIDataType::UIntPtr
            | DartFFIDataType::UInt8
            | DartFFIDataType::Uint16
            | DartFFIDataType::UInt32
            | DartFFIDataType::UInt64
            | DartFFIDataType::Float
            | DartFFIDataType::Double
            | DartFFIDataType::Bool => false,
        }
    }
}

impl Display for DartFFIDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DartFFIDataType::Char => f.write_str("ffi.Char"),
            DartFFIDataType::Void => f.write_str("ffi.Void"),
            DartFFIDataType::NativeFinalizer => f.write_str("ffi.NativeFinalizer"),
            DartFFIDataType::NativeType => f.write_str("ffi.NativeType"),
            DartFFIDataType::Opaque(_) => f.write_str("ffi.Opaque"),
            DartFFIDataType::Struct(ident) => f.write_fmt(format_args!("{}", ident.dart_label,)),
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
    CustomEnum(String),
    CustomClass(String),
    List {
        sub_type: Box<DartNativeDataType>,
    },
    Bool,
    Uri,
    Duration,
    DateTime,
    Map {
        key_type: Box<DartNativeDataType>,
        value_type: Box<DartNativeDataType>,
    },
    Void,
}

impl From<&MetaValue> for DartNativeDataType {
    fn from(meta_value: &MetaValue) -> Self {
        if meta_value.is_datetime {
            DartNativeDataType::DateTime
        } else if meta_value.is_duration {
            DartNativeDataType::Duration
        } else if meta_value.is_hashmap {
            DartNativeDataType::Map {
                key_type: Box::new(DartNativeDataType::String),
                value_type: Box::new(DartNativeDataType::String),
            }
        } else if meta_value.is_list {
            DartNativeDataType::List {
                sub_type: Box::new(DartNativeDataType::String),
            }
        } else if meta_value.is_string {
            DartNativeDataType::String
        } else if meta_value.is_timestamp {
            DartNativeDataType::DateTime
        } else if meta_value.is_url {
            DartNativeDataType::Uri
        } else if meta_value.is_error {
            DartNativeDataType::String
        } else {
            panic!(
                "No known dart native type for this Meta Type: {:#?}",
                meta_value,
            )
        }
    }
}

impl From<&CVariableDeclaration> for DartNativeDataType {
    fn from(c: &CVariableDeclaration) -> Self {
        match &c.meta {
            None => DartNativeDataType::from(&c.variable_type),
            Some(meta_value) => {
                if c.variable_type.is_struct {
                    match &c.variable_type.kind {
                        CType::Struct(s) => DartNativeDataType::CustomClass(
                            DartIdentifier::make_label_for_custom_type(&s.identifier.label),
                        ),
                        _ => panic!("Expected to extract type name of struct, got something else"),
                    }
                } else {
                    DartNativeDataType::from(meta_value)
                }
            }
        }
    }
}

impl From<&CVariableType> for DartNativeDataType {
    fn from(c: &CVariableType) -> Self {
        DartNativeDataType::from(&c.kind)
    }
}

impl DartNativeDataType {
    fn requires_pointer(&self) -> bool {
        match &self {
            DartNativeDataType::String
            | DartNativeDataType::Uri
            | DartNativeDataType::DateTime
            | DartNativeDataType::CustomClass(_)
            | DartNativeDataType::Map {
                key_type: _,
                value_type: _,
            }
            | DartNativeDataType::List { sub_type: _ } => true,

            DartNativeDataType::Bool
            | DartNativeDataType::Int
            | DartNativeDataType::Double
            | DartNativeDataType::CustomEnum(_)
            | DartNativeDataType::Duration => false,
            DartNativeDataType::Void => {
                panic!("Cannot test pointerness of this data type: {}", &self)
            }
        }
    }
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
            CType::Enum(v) => DartNativeDataType::CustomEnum(
                DartIdentifier::make_label_for_custom_type(&v.identifier.label),
            ),
            CType::Struct(v) => DartNativeDataType::CustomClass(
                DartIdentifier::make_label_for_custom_type(&v.identifier.label),
            ),
            CType::Void => DartNativeDataType::Void,
            CType::Include(_)
            | CType::Function(_)
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
            DartNativeDataType::Bool => f.write_str("bool"),
            DartNativeDataType::Int => f.write_str("int"),
            DartNativeDataType::String => f.write_str("String"),
            DartNativeDataType::Double => f.write_str("double"),
            DartNativeDataType::CustomClass(c) => f.write_str(c),
            DartNativeDataType::CustomEnum(c) => f.write_str(c),
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
            DartNativeDataType::Void => f.write_str("void"),
        }
    }
}

#[derive(Debug, Clone)]
enum DartDataType {
    NativeType(DartNativeDataType),
    FFIType(DartFFIDataType),
}

impl DartDataType {
    /// Whether the underlying data type requires wrapping in Pointer<xyz>
    fn requires_pointer(&self) -> bool {
        match &self {
            DartDataType::NativeType(nt) => nt.requires_pointer(),
            DartDataType::FFIType(ft) => ft.requires_pointer(),
        }
    }

    /// Transforms Dart Types into types that can be passed onto an FFIStruct
    /// i.e., int8 => int
    /// float => double
    /// char => Pointer<Char>, etc
    fn for_struct(&self) -> Self {
        match &self {
            DartDataType::NativeType(nt) => self.to_owned(),
            DartDataType::FFIType(ft) => match ft {
                DartFFIDataType::Int8
                | DartFFIDataType::Int16
                | DartFFIDataType::Int32
                | DartFFIDataType::Int64
                | DartFFIDataType::UIntPtr
                | DartFFIDataType::UInt8
                | DartFFIDataType::Uint16
                | DartFFIDataType::UInt32
                | DartFFIDataType::UInt64 => DartDataType::NativeType(DartNativeDataType::Int),
                DartFFIDataType::Float | DartFFIDataType::Double => {
                    DartDataType::NativeType(DartNativeDataType::Double)
                }
                DartFFIDataType::Char => DartDataType::FFIType(DartFFIDataType::Pointer {
                    sub_type: Box::new(DartFFIDataType::Char),
                }),
                DartFFIDataType::Pointer { sub_type: _ }
                | DartFFIDataType::NativeType
                | DartFFIDataType::Bool
                | DartFFIDataType::Opaque(_)
                | DartFFIDataType::Struct(_)
                | DartFFIDataType::Handle
                | DartFFIDataType::NativeFunction { sub_type: _ }
                | DartFFIDataType::Void => self.to_owned(),
                DartFFIDataType::NativeFinalizer => {
                    panic!("Native Finalizer not valid on for_struct")
                }
            },
        }
    }

    /// Deconstructs a Complex type (i.e., Duration, DateTime) into its underlying type (i.e, Duration -> int, Url -> String)
    fn to_primitive(&self) -> Self {
        match &self {
            DartDataType::NativeType(nt) => match &nt {
                DartNativeDataType::CustomClass(_)
                | DartNativeDataType::String
                | DartNativeDataType::Int
                | DartNativeDataType::Bool
                | DartNativeDataType::Void
                | DartNativeDataType::Map {
                    key_type: _,
                    value_type: _,
                }
                | DartNativeDataType::Double => self.clone(),
                DartNativeDataType::Uri => DartDataType::NativeType(DartNativeDataType::String),
                DartNativeDataType::Duration
                | DartNativeDataType::DateTime
                | DartNativeDataType::CustomEnum(_) => {
                    DartDataType::NativeType(DartNativeDataType::Int)
                }
                DartNativeDataType::List { sub_type: st } => {
                    let sst = *st.to_owned();
                    DartDataType::NativeType(sst)
                }
            },
            DartDataType::FFIType(f) => self.clone(),
        }
    }

    /// Converts a simple type into its complex form via the Meta object provided
    /// i.e., String + IsUrl => Uri
    /// ffi.Char -> String, etc
    fn to_upper_type(&self, meta: Option<&MetaValue>) -> Self {
        DartDataType::NativeType(match &meta {
            Some(m) => {
                if m.is_url {
                    DartNativeDataType::Uri
                } else if m.is_datetime {
                    DartNativeDataType::DateTime
                } else if m.is_duration {
                    DartNativeDataType::Duration
                } else if m.is_string {
                    DartNativeDataType::String
                } else if m.is_list {
                    let inner_type: DartNativeDataType = match &self {
                        /* already native type, skip */
                        DartDataType::NativeType(nt) => nt.to_owned(),
                        /* auto convert to native type */
                        DartDataType::FFIType(ft) => ft.into(),
                    };
                    DartNativeDataType::List {
                        sub_type: Box::new(inner_type),
                    }
                } else if m.is_hashmap {
                    // TODO(nf, 04/20/23): Currently the MetaValues only accept string keys
                    let key = DartNativeDataType::String;
                    let value = DartNativeDataType::String;
                    DartNativeDataType::Map {
                        key_type: Box::new(key),
                        value_type: Box::new(value),
                    }
                } else {
                    match &self {
                        /* already native type, skip */
                        DartDataType::NativeType(nt) => nt.to_owned(),
                        /* auto convert to native type */
                        DartDataType::FFIType(ft) => ft.into(),
                    }
                }
            }
            None => {
                match &self {
                    /* already native type, skip */
                    DartDataType::NativeType(nt) => nt.to_owned(),
                    /* auto convert to native type */
                    DartDataType::FFIType(ft) => ft.into(),
                }
            }
        })
    }

    fn from_meta(
        label: &str,
        cvariable: &CVariableType,
        meta: &Option<MetaValue>,
        for_ffi: bool,
    ) -> Self {
        if for_ffi {
            DartDataType::from(cvariable)
        } else {
            match meta {
                Some(m) => {
                    if cvariable.is_struct {
                        let label = if let CType::Struct(s) = &cvariable.kind {
                            &s.identifier.label
                        } else {
                            label
                        };
                        DartDataType::NativeType(DartNativeDataType::CustomClass(
                            DartIdentifier::make_label_for_custom_type(label),
                        ))
                    } else if let CType::Enum(e) = &cvariable.kind {
                        DartDataType::NativeType(DartNativeDataType::CustomEnum(
                            DartIdentifier::make_label_for_custom_type(&e.identifier.label),
                        ))
                    } else {
                        DartDataType::NativeType(DartNativeDataType::from(m))
                    }
                }
                .to_upper_type(meta.as_ref()),
                None => DartDataType::NativeType(DartNativeDataType::from(cvariable)),
            }
        }
    }
}

impl From<&CVariableType> for DartDataType {
    fn from(c: &CVariableType) -> Self {
        let return_is_ffi = c.is_struct || c.pointer_count > 0;
        if return_is_ffi {
            let mut pointer_count = c.pointer_count;
            let mut ffi_type = DartFFIDataType::from(&c.kind);
            while pointer_count > 0 {
                ffi_type = DartFFIDataType::Pointer {
                    sub_type: Box::new(ffi_type),
                };
                pointer_count -= 1;
            }
            DartDataType::FFIType(ffi_type)
        } else {
            DartDataType::NativeType(DartNativeDataType::from(&c.kind))
        }
    }
}

impl Display for DartDataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match &self {
                DartDataType::NativeType(data_type) => data_type.to_string(),
                DartDataType::FFIType(data_type) => data_type.to_string(),
            }
        ))
    }
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
            "'package:ffi/ffi.dart'",
            "'dart:ffi' as ffi",
            "'dart:io' show Platform, Directory",
            "'dart:isolate'",
            "'package:path/path.dart' as path",
        ],
    );

    let mut tera = Tera::default();
    tera.register_tester("some", is_some);

    let template_tuples = vec![
        ("generated_header", TEMPLATE_GENERATED_HEADER),
        ("dart_header", TEMPLATE_DART_HEADER),
        ("dart_library_exception", TEMPLATE_LIBRARY_EXCEPTION),
        ("c_constants", TEMPLATE_C_CONSTANTS),
        ("dart_constants", TEMPLATE_DART_CONSTANTS),
        ("dart_enums", TEMPLATE_DART_ENUMS),
        ("dart_ffi_structs", TEMPLATE_FFI_STRUCTS),
        ("dart_ffi_functions", TEMPLATE_FFI_FUNCTIONS),
        ("dart_classes", TEMPLATE_DART_CLASSES),
        ("dart_native_free_functions", TEMPLATE_DART_NATIVE_FUNCTIONS),
        ("dart_pointer_for_type", TEMPLATE_POINTER_FOR_TYPE),
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
 import {{ using }};
{% endfor %}
{% endif %}
";

const TEMPLATE_LIBRARY_EXCEPTION: &str = "
class {{meta.library_name}}Exception implements Exception {
    final String msg;
    final int code;

    const {{meta.library_name}}Exception(this.msg, this.code);
}
";

const TEMPLATE_C_CONSTANTS: &str = "
/* Region: C Constants */
const int C_NULL = 0;
const int C_TRUE = 1;
const int C_FALSE = 0;
";

const TEMPLATE_DART_CONSTANTS: &str = "
{% if constants | length %}
/* Region: Dart Constants */
{% for constant in constants %}
{% if constant.dart_comment is some %}{{ constant.dart_comment }}{% endif %}
const {{ constant.data_type }} {{ constant.identifier.dart_label }} = {{ constant.value }};
{% endfor %}
{% endif %}
";

const TEMPLATE_FFI_STRUCTS: &str = "
{% if ffi_structs | length %}
/* Region: FFI Structs */
{% for ffi_struct in ffi_structs %}

class {% if ffi_struct.is_private %}_{% endif %}{{ ffi_struct.label }} {% if ffi_struct.extends | length %} extends {% for extender in ffi_struct.extends %} {{ extender }}{% if not loop.last %}, {% endif %} {% endfor %} {% endif %}  {% if ffi_struct.implements | length %} implements {% for implementer in ffi_struct.implements %} {{ implementer }}{% if not loop.last %}, {% endif %} {% endfor %} {% endif %}{
    {% for field in ffi_struct.fields %}
    {% if field.comment is some %}{{ field.comment }}{% endif %}
    {% for annotation in field.annotations %}{{ annotation }}
    {% endfor %}
    {% for modifier in field.modifiers %} {{ modifier }} {% endfor %}{{ field.as_primitive_kind }} {% if field.is_private %}_{% endif %}{{field.identifier.dart_label}};
    {% endfor %}
}
{% endfor %}
{% endif %}
";

const TEMPLATE_DART_ENUMS: &str = "
{% block title %}{% endblock %}
{% if enums | length %}
/* Region: FFI Enums */

abstract class _IAsFFIInt {
    int get getValueAsInt;
  }

{% for enum in enums %}
{% if enum.dart_comment is some %}{{ enum.dart_comment }}{% endif %}
enum {% if enum.is_private %}_{% endif %}{{ enum.name }} implements _IAsFFIInt {
    {% for value in enum.values %}
    {% if value.dart_comment is some %}{{ value.dart_comment }}{% endif %}
    {{value.label}}({{ value.value }}),
    {% endfor %}
    ;

    final int value;

    const {{enum.name}}(this.value);

    @override
    int get getValueAsInt => value;
}
{% endfor %}
{% endif %}
";

const TEMPLATE_FFI_FUNCTIONS: &str = "
{% block title %}{% endblock %}
{% if ffi_functions | length %}
/* Region: FFI Free Functions */
{% for ffi_function in ffi_functions %}
{% if ffi_function.dart_comment is some %}{{ ffi_function.dart_comment }}{% endif %}
{{ ffi_function.return_type }} {% if ffi_function.is_private %}_{% endif %}{{ffi_function.identifier.dart_label}}(
    {% for parameter in ffi_function.parameters %}
    {% if parameter.is_required %}required {% endif %}{{ parameter.kind }} {{ parameter.identifier.dart_label }},
    {% endfor %}
) {
    {% if ffi_function.body is some %}
    {{ ffi_function.body }}
    {% else %}
    return _{{ffi_function.identifier.dart_label}}(
        {% for parameter in ffi_function.parameters %}
        {{ parameter.as_ffi_value }},
        {% endfor %}
    );
    {% endif %}
}

final {{ ffi_function.identifier.dart_label }}Ptr = _lookup<ffi.NativeFunction<{{ ffi_function.ffi_return_type }} Function({% for parameter in ffi_function.parameters %} {{ parameter.ffi_kind }}, {% endfor %}) >>('{{ ffi_function.c_function_name }}');
final _{{ ffi_function.identifier.dart_label }} = {{ ffi_function.identifier.dart_label }}Ptr.asFunction<{{ ffi_function.return_type }} Function({% for parameter in ffi_function.parameters %} {{ parameter.as_primitive_kind }}, {% endfor %}) >();
{% endfor %}
{% endif %}
";

const TEMPLATE_DART_CLASSES: &str = "
{% block title %}{% endblock %}
{% if dart_classes | length %}
/* Region: Dart Classes for use by the end-user */
    {% for class in dart_classes %}
class {% if class.is_private %}_{% endif %}{{ class.identifier.dart_label}} {% if class.implements | length %} implements {% for implementer in class.implements %} {{ implementer }}{% if not loop.last %}, {% endif %} {% endfor %} {% endif %} {% if class.extends | length %} extends {% for extender in class.extends %} {{ extender }}{% if not loop.last %}, {% endif %} {% endfor %} {% endif %} {
    {% if class.fields | length %}
    /* Fields */
    {% for field in class.fields %}
    {% if field.comment is some %}{{ field.comment }}{% endif %}
    {% for annotation in field.annotations %}{{ annotation }}
    {% endfor %}
    {% for modifier in field.modifiers %} {{ modifier }} {% endfor %}{{ field.kind }}{% if field.is_nullable %}?{% endif %} {% if field.is_private %}_{% endif %}{{field.identifier.dart_label}}{% if field.assign_statement is some %} = {{ field.assign_statement }}{% endif %};
    {% endfor %}
    {% endif %}

    {% if class.constructors | length %}
        /* Constructors */
        {% for constructor in class.constructors %}
        {% if class.is_private %}_{% endif %}{{ class.identifier.dart_label }}.{% if constructor.is_private %}_{% endif %}{{constructor.identifier.dart_label}}({% for parameter in constructor.parameters %}this.{{ parameter }}, {% endfor %}) {% if constructor.body is some %} {
            {{ constructor.body }}
        }{% else %};{% endif %}
        {% endfor %}
    {% endif %}

    {% if class.functions | length %}
    /* Functions */
    {% for function in class.functions %}
        {% if function.dart_comment is some %}{{ function.dart_comment }}{% endif %}
        {% for annotation in function.annotations %}{{ annotation }}{% endfor %}
        {% for modifier in function.modifiers %}{{ modifier }} {% endfor %} {% if function.is_async %} {# Future< #}{% endif %}{% if function.is_factory %} {% elif function.is_void %} void {% else %} {{ function.return_type }} {% endif %}{% if function.is_async %}{# > #}{% endif %} {% if function.is_factory %}{{ class.identifier.dart_label }}.{% endif %}{% if function.is_private %}_{% endif %}{{ function.identifier.dart_label}}({% for parameter in function.parameters %} {% if parameter.is_required %}required {% endif %}{{ parameter.kind }} {{ parameter.identifier.dart_label}}, {% endfor %}) {% if function.is_async %} {# async #} {% endif %} {
            {% if function.is_async %} {# return await Isolate.run(() { #} {% endif %}
            {% if function.body is some %}
        {{ function.body }}
            {% elif function.identifier.dart_label is containing(\"fromCStruct\") %}
                {% for field in class.fields %}
        final {{ field.kind }}{% if field.is_nullable %}?{% endif %} _c{{field.identifier.dart_label}} = _transformFromFFI<{{ field.kind }}>(c.{{field.identifier.dart_label}}, {% if field.meta is some %} {% if field.meta.is_list %}isList: true, {% elif field.meta.is_url %}isUri: true,{% elif field.meta.is_duration%}isDuration: true, {% elif field.meta.is_datetime%}isDateTime: true,  {% endif %}{% endif %} {% if field.reads_length_from is some %} listSize: c.{{ field.reads_length_from }}, {% endif %}){% if not field.is_nullable %}!{% endif %};
                {% endfor %}
            final _{{class.identifier.dart_label}}Ret = {{ class.identifier.dart_label }}._fromFields({% for field in class.fields %}_c{{ field.identifier.dart_label }}, {% endfor %});
            return _{{class.identifier.dart_label}}Ret;
            {% elif function.meta is some and function.meta.is_constructor %}
                final c{{function.c_function_name}}OutputPtr = _getPointerForType<{{ class.identifier.dart_label }}>();
                {% if function.throws %}
                final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr = _getPointerForType<String>().cast();
                {% endif %}
                {% for parameter in function.parameters %}
                final dynamic c{{ parameter.identifier.dart_label }}ForFFI = _transformToFFI({{ parameter.identifier.dart_label }});
                {% endfor %}
                /* call native function */
                int errCode = ffi_{{ function.c_function_name }}(
                        c{{function.c_function_name}}OutputPtr.cast(),
                        {% for parameter in function.parameters %}
                        c{{ parameter.identifier.dart_label }}ForFFI,
                        {% endfor %}
                    cErrPtr,
                );

                /* Check error code */
                if (errCode != C_FALSE) {
                    /* free pointers if required  */
                    {% for parameter in function.parameters %}
                        {% if parameter.requires_pointer and not parameter.is_persistent %}
                    calloc.free(c{{ parameter.identifier.dart_label }}ForFFI);
                        {% endif %}
                    {% endfor %}
                    {% if function.output_requires_pointer %}
                    calloc.free(c{{function.c_function_name}}OutputPtr);
                    {% endif %}

                    /* throw final Exception */
                    throw {{meta.library_name}}Exception(_getDartStringFromDoublePtr(cErrPtr), errCode);
                }

                /* Free allocated pointers */
                    {% for parameter in function.parameters %}
                        {% if parameter.requires_pointer and not parameter.is_persistent %}
                calloc.free(c{{ parameter.identifier.dart_label}}ForFFI);
                        {% endif %}
                    {% endfor %}
                    {% if function.throws %}
                calloc.free(cErrPtr);
                    {% endif %}

                /* return final value */
                return  {{ class.identifier.dart_label }}._fromCPointerPointer(c{{function.c_function_name}}OutputPtr.cast());
            {% else %}
            /* Instance Method */
                {% if function.throws %}
            /* Get error pointer in case function returns failure */
            final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr = _getPointerForType<String>().cast();
                {% endif %}
                {% if not function.is_void %}
                    {% if function.output_requires_pointer %}
                final _c{{ function.c_function_name }}OutputPtr = _getPointerForType<{{ function.return_type }}>().cast();
                    {% endif %}
                {% endif %}
                {% for parameter in function.parameters %}
            final c{{ parameter.identifier.dart_label }}ForFFI = _transformToFFI({{ parameter.identifier.dart_label }});
                {% endfor %}
                /* call native function */
                int errCode = ffi_{{ function.c_function_name }}(
                        this._selfPtr,
                        {% if function.output_requires_pointer %}_c{{ function.c_function_name }}OutputPtr.cast(),{% endif %}
                        {% for parameter in function.parameters %}
                        c{{ parameter.identifier.dart_label }}ForFFI,
                        {% endfor %}
                    cErrPtr,
                );

                /* Check error code */
                if (errCode != C_FALSE) {
                    /* free pointers if required  */
                    {% for parameter in function.parameters %}
                        {% if parameter.requires_pointer and not parameter.is_persistent %}
                    calloc.free(c{{ parameter.identifier.dart_label }}ForFFI);
                        {% endif %}
                    {% endfor %}
                    {% if function.output_requires_pointer %}
                    calloc.free(_c{{function.c_function_name}}OutputPtr);
                    {% endif %}

                    /* throw final Exception */
                    throw {{meta.library_name}}Exception(_getDartStringFromDoublePtr(cErrPtr), errCode);
                }

                /* Free allocated pointers */
                    {% for parameter in function.parameters %}
                        {% if parameter.requires_pointer and not parameter.is_persistent %}
                calloc.free(c{{ parameter.identifier.dart_label}}ForFFI);
                        {% endif %}
                    {% endfor %}
                    {% if function.throws %}
                calloc.free(cErrPtr);
                    {% endif %}
        'TODO: ayy lmao put that body here boi';
            {% if not function.is_void %}
        /* return final value */
                {% if function.output_requires_pointer %}
                    // requires a pointer bitches
                    {% if function.return_type_struct %}
        return {{ function.return_type }}._fromCPointerPointer(_c{{function.c_function_name}}OutputPtr.cast());
                    {% else %}
        return _transformFromFFI(_c{{function.c_function_name}}OutputPtr, isDoublePointer: true)!;
                    {% endif %}
                {% else %}
                return ??? // todo: what do when function is not void, and doesnt return a struct;
                {% endif %}
            {% endif %}
        {% endif %}
        {% if function.is_async %} {# }); #} {% endif %}

    }
    {% endfor %}
    {% endif %}
}
    {% endfor %}
{% endif %}
";

const TEMPLATE_DART_NATIVE_FUNCTIONS: &str = "
{% block title %}{% endblock %}
{% if dart_native_free_functions | length %}
/* Region: Dart Free Functions */
{% for function in dart_native_free_functions %}
{% for annotation in function.annotations %}{{ annotation }}{% endfor %}
{% for modifier in function.modifiers %} {{ modifier }} {% endfor %} {% if function.is_async %} {# Future< #}{% endif %}{{ function.return_type }} {% if function.is_async %}{# > #}{% endif %} {% if function.is_private %}_{% endif %}{{ function.identifier.dart_label}}({% for parameter in function.parameters %} {% if parameter.is_required %}required {% endif %}{{ parameter.kind }} {{ parameter.identifier.dart_label }}, {% endfor %}) {% if function.is_async %} {# async #} {% endif %}{
    {% if function.is_async %}
    {# return await Isolate.run(() { #} 
    {% endif %}
    {% if function.body is some %}
    {{ function.body }}
    {% else %}
        {% if function.throws %}
    /* Get error pointer in case function returns failure */
    final ffi.Pointer<ffi.Pointer<ffi.Char>> cErrPtr = _getPointerForType<String>().cast();

            {% if function.parameters | length %}
    /* get pointer types for items that require it*/
                {% for parameter in function.parameters %}
                    {% if parameter.requires_pointer %}
    final c{{ parameter.identifier.dart_label}}Ptr = _getPointerForData({{ parameter.identifier.dart_label }});
                    {% endif %}
                {% endfor %}
            {% endif %}

            {% if function.output_requires_pointer %}
    /* get Output Pointer type */
    final _c{{function.c_function_name}}OutputPtr = _getPointerForType<{{ function.return_type }}>();
            {% endif %}

    /* call native function */
    int errCode = ffi_{{ function.identifier.dart_label }}(
            {% if function.output_requires_pointer %}
        _c{{function.c_function_name}}OutputPtr.cast(),
            {% endif %}
            {% for parameter in function.parameters %}
                {% if parameter.requires_pointer %} c{{ parameter.identifier.dart_label }}Ptr.cast(){% else %} {{ parameter.identifier.dart_label }}{% endif %},
            {% endfor %}
        cErrPtr,
    );

    /* Check error code */
    if (errCode != C_FALSE) {
        /* free pointers if required  */
        {% for parameter in function.parameters %}
            {% if parameter.requires_pointer and not parameter.is_persistent %}
        calloc.free(c{{ parameter.identifier.dart_label }}Ptr);
            {% endif %}
        {% endfor %}
        {% if function.output_requires_pointer %}
        calloc.free(_c{{function.c_function_name}}OutputPtr);
        {% endif %}

        /* throw final Exception */
        throw {{meta.library_name}}Exception(_getDartStringFromDoublePtr(cErrPtr), errCode);
    }

    /* Free allocated pointers */
        {% for parameter in function.parameters %}
            {% if parameter.requires_pointer and not parameter.is_persistent %}
    calloc.free(c{{ parameter.identifier.dart_label}}Ptr);
            {% endif %}
        {% endfor %}
        {% if function.throws %}
    calloc.free(cErrPtr);
        {% endif %}

    {% if not function.is_void %}
        // not void bitches
    /* return final value */
            {% if function.output_requires_pointer %}
                // requires a pointer bitches
                {% if function.return_type_struct %}
    return {{ function.return_type }}._fromCPointerPointer(_c{{function.c_function_name}}OutputPtr.cast());
                {% else %}
    return _transformFromFFI(_c{{function.c_function_name}}OutputPtr, isDoublePointer: true)!;
                {% endif %}
            {% else %}
            return ??? // todo: what do when function is not void, and doesnt return a struct;
            {% endif %}
        {% endif %}
    {% else %}
    \'lol?\';
    {% endif %}
    {% endif %}
    {% if function.is_async %}
    {# }); #}
    {% endif %}
}
{% endfor %}
{% endif %}
";

const TEMPLATE_POINTER_FOR_TYPE: &str = "
/* Region: Dart Pointer Utility Functions  */

/// Interface to get a Pointer to the backing data on classes that
/// cross FFI boundaries
abstract class _IWithPtr {
    ffi.Pointer<ffi.Void> getPointer();
}

/// Holds the symbol lookup function.
final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup = loadLibrary(getLibraryPath()).lookup;


dynamic _transformToFFI<T>(T upperData) {
    if (T == String) {
        return _stringToFFIPointer(upperData as String);
      } else if (T == int || T == double) {
        return upperData;
      } else if (T == bool) {
        return upperData as bool;
      } else if (T == Uri) {
        final s = (upperData as Uri).toString();
        return _stringToFFIPointer(s);
      } else if (T == Duration) {
        return (upperData as Duration).inMilliseconds;
      } else if (T == DateTime) {
        return (upperData as DateTime).millisecondsSinceEpoch;
      } else if (T == List<String>) {
        // TODO(nf, 04/20/23): This join is only valid for OAuth Space-Separated lists.
        // It is not general.
        final s = (upperData as List<String>).join(' ');
        return _stringToFFIPointer(s);
      } else if (T == Map<String, String>) {
        // TODO(nf, 04/20/23): This is only valid for String:String pairs
        // It is not
        return _hashMapToFFIPointer(upperData as Map<String, String>);
      } else if (T == _IWithPtr) {
        return _getPointerForData(upperData);
      } else if (T == ffi.Pointer) {
        return upperData as ffi.Pointer;
      } else if (upperData is _IAsFFIInt) {
        return (upperData as _IAsFFIInt);
      } else {
        throw {{meta.library_name}}Exception('Invalid ffi data, cannot transform into FFI for for $T', -5);
      }
  }


  T? _transformFromFFI<T>(dynamic data, {bool isDoublePointer = false, bool isList = false, bool isHashMap = false, bool isUri = false, bool isDuration = false, bool isDateTime = false, int listSize = 0}) {
    if ((T == double && (data is double || data is int)) || (T == int && data is int)) {
        if (isDuration) {
        return Duration(milliseconds: data.toInt()) as T;
      } else if (isDateTime) {
        return DateTime.fromMillisecondsSinceEpoch(data.toInt()) as T;
      } else {
        return data as T;
      }
    } else if (T == bool && data is bool) {
      return data as T;
    } else if (T == String && data is String) {
      return data as T;
    } else if (data is ffi.Pointer) {
      if (data.address == ffi.nullptr.address) {
        return null;
      } else if (isList) {
        // TODO(nf, 04/22/23): we assume lists are List<String> and are single pointer lists
        ffi.Pointer<ffi.Pointer<ffi.Char>> ptr = data.cast();
        List<String> lst = [];
        for (int i = 0; i < listSize; i++) {
          final charPtr = ptr.elementAt(i).value;
          final s = _getDartStringFromPtr(charPtr);
          lst.add(s);
        }
        return lst as T;
    } else if (T == Uri || T == String) {
        late final String s;
        if (isDoublePointer) {
          s = _getDartStringFromDoublePtr(data.cast());
        } else {
          s = _getDartStringFromPtr(data.cast());
        }
        if (isUri) {
          final uri = Uri.tryParse(s);
          if (uri == null) {
            throw {{meta.library_name}}Exception('Failed during parsing of URI. Got invalid string: $s', -7);
          } else {
            return uri as T;
          }
        } else {
          return s as T;
        }
      } else if (data is ffi.Pointer<ffi.Pointer<ffi.NativeType>>) {
        return transformFromPointer<T, ffi.NativeType>(data, isDoublePointer);
      }
    }
    throw {{meta.library_name}}Exception('Invalid data in transformFromFFI: $data', -4);
  }

T transformFromPointer<T, E extends ffi.NativeType>(ffi.Pointer<E> data, bool isDoublePointer) {
    if (T == String) {
        if(isDoublePointer) {
            return _getDartStringFromDoublePtr(data.cast()) as T;
        } else {
            return _getDartStringFromPtr(data.cast()) as T;
        }
    }
    {% for class in dart_classes %}
    else if(T == {{ class.backing_ffi_struct.dart_label }} || T == {{ class.identifier.dart_label }}) {
        if(isDoublePointer) {
            return {{ class.identifier.dart_label }}._fromCPointerPointer(data.cast()) as T;
        } else {
            return {{ class.identifier.dart_label }}._fromCPointer(data.cast()) as T;
        }
    }
    {% endfor %}
    throw {{meta.library_name}}Exception('Invalid data in transformFromPointer: $T', -4);
  }

ffi.Pointer<ffi.Void> _getPointerForType<T>() {
    if (T == String) {
      return _getEmptyStringPointer().cast();
    }
    {% for class in dart_classes %}
    else if(T == {{ class.identifier.dart_label }} || T == {{ class.backing_ffi_struct.dart_label }}) {
        return calloc<ffi.Pointer<{{ class.backing_ffi_struct.dart_label }}>>().cast();
    }
    {% endfor %}
    else {
      throw {{meta.library_name}}Exception('Invalid type: $T', -3);
    }
  }

  /// Returns a castable pointer based on the input data.
/// This function is only valid for Types [String, {custom C generated classes}]
/// Will throw an Exception if passed invalid types
ffi.Pointer<ffi.Void> _getPointerForData(dynamic data) {
    if (data is String) {
      return _stringToFFIPointer(data).cast();
    } else if (data is _IWithPtr) {
      return data.getPointer().cast();
    } else {
      throw {{meta.library_name}}Exception('Invalid data type for pointer: $data', -2);
    }
  }


/// Returns a Dart String from a `char*`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For double pointers `char**` use `_getDartStringFromDoublePtr`
String _getDartStringFromPtr(ffi.Pointer<ffi.Char> charPtr) {
  final asUtf8Ptr = charPtr.cast<Utf8>();
  final asDartString = asUtf8Ptr.toDartString();
  calloc.free(charPtr);
  return asDartString;
}

/// Returns a Dart String from a `char**`
///
/// n.b., THIS CONSUMES AND FREES THE POINTER
/// Do not use `charPtr` after this
///
/// For single pointers `char*` use `_getDartStringFromPtr`
String _getDartStringFromDoublePtr(ffi.Pointer<ffi.Pointer<ffi.Char>> doublePtr) {
  final asCharPtr = doublePtr.value.cast<ffi.Char>();
  final dstr = _getDartStringFromPtr(asCharPtr);
  calloc.free(doublePtr);
  return dstr;
}

ffi.Pointer<ffi.Char> _stringToFFIPointer(String s) {
  return s.toNativeUtf8().cast<ffi.Char>();
}

ffi.Pointer<ffi.Char> _hashMapToFFIPointer(Map<String, String> dict) {
  String val = dict.entries.map((e) => \"${e.key}:${e.value}\").join(';');
  return _stringToFFIPointer(val);
}

/// Returns the Dart equivalent of an empty `char**`
ffi.Pointer<ffi.Pointer<ffi.Char>> _getEmptyStringPointer() {
  return calloc<ffi.Pointer<ffi.Char>>();
}
";

const TEMPLATE_UTILITY_FUNCTIONS: &str = "
/* Region: Utility Functions  */

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
ffi.DynamicLibrary loadLibrary(String libraryPath) {
  final dylib = ffi.DynamicLibrary.open(libraryPath);
  return dylib;
}
";
