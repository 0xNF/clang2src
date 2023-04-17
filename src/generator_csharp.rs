use std::{collections::HashMap, fmt::Display};

use serde::Serialize;

use crate::lexer::{
    CEnum, CFunction, CIdentifier, CStruct, CType, CVariableDeclaration, HeaderFile,
};
use crate::meta::{MetaValue, META_PARAM_TOKEN, META_TOKEN};

#[derive(Serialize)]
struct Data<'a> {
    dll_location: &'a str,
    usings: Vec<&'a str>,
    namespace: &'a str,
    constants: Vec<CSharpVariable>,
    enums: Vec<CSharpEnum>,
    structs: Vec<CSharpStruct>,
    functions: Vec<CSharpFunction>,
}

impl<'a> Data<'a> {
    fn new(
        dll_location: &'a str,
        usings: Vec<&'a str>,
        namespace: &'a str,
        constants: Vec<CSharpVariable>,
        enums: Vec<CSharpEnum>,
        structs: &mut Vec<CSharpStruct>,
        functions: &mut Vec<CSharpFunction>,
    ) -> Self {
        // /* attach functions to structs */
        for f in functions.iter_mut() {
            if let Some(cmt) = &f.c_comment {
                let splits = cmt.split('\n').into_iter();
                for split in splits {
                    if split.contains("#meta:") {
                        f.meta = MetaValue::from_meta_comment(split);
                    }
                }
            }
        }
        for f in functions.iter_mut() {
            if let Some(meta) = &f.meta {
                if meta.for_struct {
                    let v: Vec<&str> = f.c_label.split("_").collect();
                    let struct_name = v.first().unwrap();
                    for _struct in structs.into_iter() {
                        if _struct.label == *struct_name {
                            let mut f_clone = f.clone();
                            let repl = format!("{}_", _struct.label);
                            let replaced = f.csharp_label.replace(&repl, "");
                            f_clone.csharp_label = replaced;
                            // f_clone.csharp_label =
                            //     (f.csharp_label).replace(&format!("{}_", _struct.label), "");
                            _struct.functions.push(f_clone);
                        }
                    }
                }
            }
        }
        Data {
            dll_location,
            usings,
            namespace,
            constants,
            enums,
            structs: structs.to_vec(),
            functions: functions.clone(),
        }
    }
}

#[derive(Serialize, Clone)]
enum DataType {
    Byte(u8),
    SByte(i8),
    _Char(String),
    String(String),
    UInt(u32),
    Int(i32),
    ULong(u64),
    Long(i64),
    Double(f64),
    Void,
    IntPtr,
}
impl From<CType> for DataType {
    fn from(src: CType) -> Self {
        match src {
            CType::SignedShort(v) => DataType::Int(v.into()),
            CType::UnsignedShort(v) => DataType::UInt(v.into()),
            CType::SignedInteger(v) => DataType::Int(v.into()),
            CType::UnsignedInteger(v) => DataType::UInt(v.into()),
            CType::SignedLong(v) => DataType::Long(v.into()),
            CType::UnsignedLong(v) => DataType::ULong(v.into()),
            CType::Int64T(v) => DataType::Long(v.into()),
            CType::Float(v) => DataType::Double(v.into()),
            CType::Double(v) => DataType::Double(v.into()),
            CType::Char(v) => DataType::String(v.into()),
            CType::IntPtrT(_) => DataType::IntPtr,
            CType::UIntPtrT(v) => DataType::ULong(v.into()),
            CType::Int8T(v) => DataType::SByte(v.into()),
            CType::Int16T(v) => DataType::Int(v.into()),
            CType::Int32T(v) => DataType::Int(v.into()),
            CType::UInt8T(v) => DataType::Byte(v.into()),
            CType::UInt16T(v) => DataType::UInt(v.into()),
            CType::UInt32T(v) => DataType::UInt(v.into()),
            CType::UInt64T(v) => DataType::ULong(v.into()),
            CType::Void => DataType::Void,
            _ => panic!("Cannot map type {} to CSharp type", src),
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataType::Byte(_) => "byte",
            DataType::SByte(_) => "sbyte",
            DataType::_Char(_) => "char",
            DataType::String(_) => "string",
            DataType::UInt(_) => "uint",
            DataType::Int(_) => "int",
            DataType::ULong(_) => "ulong",
            DataType::Long(_) => "long",
            DataType::Double(_) => "double",
            DataType::IntPtr => "IntPtr",
            DataType::Void => "void",
        };
        f.write_str(s)
    }
}

#[derive(Serialize, Clone)]
struct CSharpStruct {
    label: String,
    c_comment: Option<String>,
    csharp_comment: Option<String>,
    members: Vec<CSharpVariable>,
    functions: Vec<CSharpFunction>,
}

impl From<&CStruct> for CSharpStruct {
    fn from(v: &CStruct) -> Self {
        CSharpStruct {
            label: v.identifier.label.to_owned(),
            c_comment: v.comment.to_owned(),
            csharp_comment: transform_comment(v.comment.to_owned()),
            members: v
                .declarations
                .iter()
                .map(|decl| CSharpVariable::from(decl))
                .collect(),
            functions: vec![],
        }
    }
}

#[derive(Serialize, Clone)]
struct CSharpFunction {
    c_label: String,
    csharp_label: String,
    c_comment: Option<String>,
    csharp_comment: Option<String>,
    return_type: CSharpVariable,
    parameters: Vec<CSharpVariable>,
    meta: Option<MetaValue>,
}
impl From<&CFunction> for CSharpFunction {
    fn from(src: &CFunction) -> Self {
        let (ret_pointer_count, ret_data_type) =
            CSharpVariable::sub_variable(&src.return_type.kind, src.return_type.pointer_count);

        let mut params: Vec<CSharpVariable> = vec![];
        let mut param2meta: HashMap<String, Option<MetaValue>> = HashMap::new();
        if let Some(cmt) = &src.comment {
            let mut iter = cmt
                .split('\n')
                .skip_while(|&x| !x.contains(META_PARAM_TOKEN));
            while let Some(c) = iter.next() {
                if !c.contains(META_PARAM_TOKEN) {
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
        for n in 0..src.parameters.len() {
            let mut v = CSharpVariable::from(&src.parameters[n]);
            if n == src.parameters.len() - 1 {
                v.is_last = true;
            }

            if let Some(meta_value) = &param2meta.get(&v.label) {
                v.meta = (*meta_value).to_owned();
            }

            params.push(v);
        }

        CSharpFunction {
            c_label: src.label.to_owned(),
            csharp_label: src.label.to_owned(),
            c_comment: src.comment.to_owned(),
            csharp_comment: transform_comment(src.comment.to_owned()),
            parameters: params,
            return_type: CSharpVariable {
                label: "".into(),
                c_comment: None,
                csharp_comment: None,
                value: None,
                data_type: ret_data_type.to_string(),
                pointer_count: ret_pointer_count,
                is_last: true,
                meta: None,
            },
            meta: None,
        }
    }
}

#[derive(Serialize, Clone)]
struct CSharpVariable {
    label: String,
    c_comment: Option<String>,
    csharp_comment: Option<String>,
    value: Option<DataType>,
    data_type: String,
    pointer_count: u8,
    is_last: bool,
    meta: Option<MetaValue>,
}
impl Display for CSharpVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stars: String = "".into();
        for _ in 0..self.pointer_count {
            stars += "*";
        }

        f.write_fmt(format_args!("{}{}", self.data_type, stars))
    }
}

impl CSharpVariable {
    fn sub_variable(var_type: &CType, pointer_count: u8) -> (u8, DataType) {
        let data_type: DataType;
        let mut adjusted_pointer_count: u8 = pointer_count;
        match var_type {
            CType::Void => data_type = DataType::Void,

            CType::Enum(_) => data_type = DataType::Int(0),

            CType::Double(v) => {
                data_type = DataType::Double(*v);
            }
            CType::Char(_) => {
                /* handle Strings (aka const char*) */
                if pointer_count > 0 {
                    data_type = DataType::IntPtr;
                    adjusted_pointer_count = pointer_count - 1;
                } else {
                    data_type = DataType::Byte(0);
                    adjusted_pointer_count = 0
                }
            }
            CType::Struct(_) => {
                data_type = DataType::IntPtr;
                if pointer_count > 0 {
                    adjusted_pointer_count = pointer_count - 1;
                }
            }
            _ => data_type = DataType::from(var_type.clone()),
        };

        (adjusted_pointer_count, data_type)
    }
}

impl From<&CVariableDeclaration> for CSharpVariable {
    fn from(src: &CVariableDeclaration) -> Self {
        let label: String = src.label.to_owned();
        let (pointer_count, data_type) =
            CSharpVariable::sub_variable(&src.variable_type.kind, src.variable_type.pointer_count);

        let mut stars: String = "".into();
        for _ in 0..pointer_count {
            stars += "*";
        }

        CSharpVariable {
            label,
            c_comment: src.comment.to_owned(),
            csharp_comment: transform_comment(src.comment.to_owned()),
            data_type: format!("{}{}", data_type, stars),
            pointer_count,
            value: Some(data_type),
            is_last: false,
            meta: None,
        }
    }
}

#[derive(Serialize)]
struct CSharpIdentifier {
    label: String,
    comment: Option<String>,
}
impl From<&CIdentifier> for CSharpIdentifier {
    fn from(src: &CIdentifier) -> Self {
        CSharpIdentifier {
            label: src.label.to_owned(),
            comment: transform_comment(src.comment.to_owned()),
        }
    }
}

#[derive(Serialize)]
struct CSharpEnum {
    label: String,
    comment: Option<String>,
    members: Vec<CSharpIdentifier>,
}

impl From<&CEnum> for CSharpEnum {
    fn from(src: &CEnum) -> Self {
        CSharpEnum {
            label: src.identifier.label.to_owned(),
            comment: transform_comment(src.comment.to_owned()),
            members: src
                .declarations
                .iter()
                .map(|decl| CSharpIdentifier::from(decl))
                .collect(),
        }
    }
}

/// Parses the various single line, multi-line paragraph comments ina  C# friendly way
fn transform_comment(cmt: Option<String>) -> Option<String> {
    if let Some(c) = cmt {
        let mut cmts: Vec<String> = vec![];
        for split in c.split("\n").map(|f| f.trim()) {
            if split.contains(META_TOKEN) {
                continue;
            }
            let mut s: String = "".into();
            if split.starts_with("//") {
                s = split.replacen("//", "/// ", 1).into();
            } else if split.starts_with("/**") {
                s = split.replacen("/**", "/// ", 1).into();
            } else if split.starts_with("/*") {
                s = split.replacen("/*", "/// ", 1).into();
            } else if split.starts_with("*/") {
                continue;
            } else if split.starts_with("*") {
                s = split.replacen("*", "/// ", 1).into();
            } else if !split.starts_with("///") {
                s = format!("/// {}", split);
            }
            s = s.trim().into();
            if s == "///" {
                continue;
            }
            s = s.trim_end_matches("*/").into();
            cmts.push(s);
        }
        if cmts.len() > 0 {
            cmts.insert(0, "/// <summary>".into());
            cmts.push("/// </summary>".into());
            Some(cmts.join("\n"))
        } else {
            None
        }
    } else {
        cmt.into()
    }
}

pub fn generate(header: HeaderFile, namespace: &str, dll_location: &str) {
    let csharp = Data::new(
        dll_location,
        vec!["System", "System.Runtime.InteropServices", "System.Linq"],
        namespace,
        header
            .defines
            .iter()
            .map(|cv| CSharpVariable::from(cv))
            .collect(),
        header.enums.iter().map(|e| CSharpEnum::from(e)).collect(),
        &mut header
            .structs
            .iter()
            .map(|s| CSharpStruct::from(s))
            .collect(),
        &mut header
            .functions
            .iter()
            .map(|f| CSharpFunction::from(f))
            .collect(),
    );

    let header = mustache::compile_str(TEMPLATE_HEADER)
        .unwrap()
        .render_to_string(&csharp)
        .unwrap();
    let classes = mustache::compile_str(TEMPLATE_CLASSES)
        .unwrap()
        .render_to_string(&csharp)
        .unwrap();
    let enums = mustache::compile_str(TEMPLATE_ENUMS)
        .unwrap()
        .render_to_string(&csharp)
        .unwrap();
    let ffi_body = mustache::compile_str(TEMPLATE_BODY)
        .unwrap()
        .render_to_string(&csharp)
        .unwrap();
    let helpers = mustache::compile_str(TEMPLATE_HELPERS)
        .unwrap()
        .render_to_string(&csharp)
        .unwrap();
    let footer = mustache::compile_str(TEMPLATE_FOOTER)
        .unwrap()
        .render_to_string(&csharp)
        .unwrap();
    let total: Vec<String> = vec![header, enums, ffi_body, helpers, footer];
    println!("{}", total.join(""));
}

const TEMPLATE_CLASSES: &str = "
#region classes
{{#structs}}
{{#csharp_comment}}{{{csharp_comment}}}{{/csharp_comment}}
public class {{label}} {
    private readonly IntPtr selfPtr;

    {{#functions}}
    {{#meta}}
    {{#is_destructor}}~{{label}}(){
        FreeStruct(selfPtr);
    }{{/is_destructor}}
    {{^is_destructor}}
    {{#is_constructor}}
    public {{label}}({{#parameters}}{{#meta}}{{^is_error}}{{data_type}} {{label}}{{^is_last}}, {{/is_last}}{{/is_error}}{{/meta}}{{/parameters}}) {
        List<IntPtr> freeMe = new List<IntPtr>();

        try {
            {{#parameters}}
            {{label}}_meta= {{#meta}}{{for_what}}{{/meta}}
            {{/parameters}}
        }
        finally
        {
            foreach(IntPtr ptr in freeMe) {
                FreeString(ptr);
            }
        }
        this.selfPtr = {{c_label}}();
    }
    {{/is_constructor}}
    {{^is_constructor}}
    {{#csharp_comment}}{{{csharp_comment}}}{{/csharp_comment}}
    public {{#meta}}{{#is_static}}static {{/is_static}}{{/meta}}void {{csharp_label}}(){}
    {{/is_constructor}}
    {{/is_destructor}}
    {{/meta}}
    {{/functions}}

    private static Tup<C_{{label}}> GetCPointers() {
        {{#functions}}
        {{#meta}}
        {{#is_constructor}}
        {{c_label}}({{#parameters}}{{data_type}} {{c_label}}{{^is_last}}, {{/is_last}}{{/parameters}});
        {{/is_constructor}}
        {{/meta}}
        {{/functions}}
        IntPtr selfPtr = f();
        C_{{label}} structItem = Marshal.PtrToStructure<C_{{label}}>(selfPtr);
        return new Tup<C_{{label}}>(structItem, selfPtr);
    }
}
{{/structs}}
#endregion
";

const TEMPLATE_HEADER: &str = "
/// THIS CODE IS AUTOGENERATED WITH clang2src
/// DO NOT MODIFY THIS FILE -- your changes will be lost when this file is regenerated

{{#usings}}
using {{.}};
{{/usings}}

namespace {{namespace}} {
";

const TEMPLATE_ENUMS: &str = "
#region enums
{{#enums}}
{{#comment}}{{{comment}}}
{{/comment}}public enum {{label}} : int {
    {{#members}}
    {{#comment}}
    {{{comment}}}
    {{/comment}}
    {{label}},
    {{/members}}
}
{{/enums}}
#endregion
";

const TEMPLATE_HELPERS: &str = "
#region helpers


internal unsafe partial class FFIInterface
{
    internal static string GetErrorMessage(IntPtr ptrErrMsg)
    {
        string errmsg = Marshal.PtrToStringAnsi(ptrErrMsg);
        if (errmsg == null)
        {
            throw new Exception(\"Failed to extract error message\");
        }
        return errmsg;
    }

    internal static string PtrToString(IntPtr ptr)
    {
        string errmsg = Marshal.PtrToStringAnsi(ptr);
        if (errmsg == null)
        {
            throw new Exception(\"Failed to extract string\");
        }
        return errmsg;
    }

    internal static IntPtr StringToFFIPointer(string s)
    {
        return Marshal.StringToHGlobalAnsi(s);
    }

    internal static void FreeString(IntPtr ptr)
    {
        Marshal.FreeHGlobal(ptr);
    }

    internal static void FreeStruct(IntPtr ptr)
    {
        Marshal.FreeHGlobal(ptr);
    }

    internal static IntPtr HashMap2IntPtr(IList<string> dict)
    {
        if (dict.Count == 0)
        {
            return StringToFFIPointer(\"\");
        }
        List<string> strings = dict.Where((x) => x.Split('=').Length == 2).ToList();
        string final = String.Join(\";\", strings);
        return StringToFFIPointer(final);
    }
}

internal class Tup<T>
{
    internal readonly T structure;
    internal readonly IntPtr ptr;

    internal Tup(T structure, IntPtr ptr)
    {
        this.structure = structure;
        this.ptr = ptr;
    }
}


#endregion
";

const TEMPLATE_BODY: &str = "
    internal unsafe partial class FFIInterface {
        #region constants
        {{#constants}}
        {{#comment}}
        {{{comment}}}
        {{/comment}}internal const {{data_type}} {{label}}{{#value}} = {{value}}{{/value}};
        {{/constants}}
        #endregion

        #region structs
        {{#structs}}
        [StructLayout(LayoutKind.Sequential)]
        {{#csharp_comment}}{{{csharp_comment}}}{{/csharp_comment}}
        internal readonly struct C_{{label}} {
            {{#members}}
            {{#csharp_comment}}{{{csharp_comment}}}{{/csharp_comment}}
            internal readonly {{data_type}} {{label}};
            {{/members}}
        }

        {{/structs}}
        #endregion

        #region functions
        {{#functions}}
        {{#csharp_comment}}{{{csharp_comment}}}{{/csharp_comment}}
        [DllImport(\"{{dll_location}}\")]
        internal static extern {{#return_type}}{{data_type}}{{/return_type}} {{c_label}}({{#parameters}}{{data_type}} {{label}}{{^is_last}}, {{/is_last}}{{/parameters}});
        {{/functions}}
        #endregion
    }

";

const TEMPLATE_FOOTER: &str = "
}
";
