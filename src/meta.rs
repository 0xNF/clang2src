use std::str::FromStr;

use fancy_regex::Regex;
use serde::Serialize;

pub const META_TOKEN: &str = "#meta:";
pub const META_PARAM_TOKEN: &str = "#meta_param:";

#[derive(Debug, Serialize, Clone)]
pub struct MetaValue {
    /// Whether this function should be considered async, and thus should final generated functions
    /// be wrapped in some kind of async handler
    pub is_async: bool,
    /// Whether this struct must maintain a reference to a backing pointer to Rust-land
    pub is_persistent: bool,

    /// Whether this is a method that belongs on struct, the struct itself being denoted by the text leading up the first underscore in the function name
    /// #meta: for_struct;
    pub for_struct: bool,

    /// if `for_strict` is set, whether this function should be a static method on the struct
    /// #meta: is_static;
    pub is_static: bool,

    /// Whether this value can be null (or Option) in languages which recognize the concept
    /// #meta_param: value_name;nullable;
    pub is_nullable: bool,

    /// Whether this value must be handed in as a Struct Ptr for the underlying C call
    /// #meta_param: value_name;as_ptr;
    pub as_ptr: bool,

    /// Whether this value should be interpreted as a List
    /// See also: `length_for` and `capacity_for`
    pub is_list: bool,

    /// Whether this value is the pointer to the object that the method this parameter is for should attach to
    pub is_this: bool,

    /// Whether ths function doesnt return anything at all (not including error codes)
    pub is_void: bool,

    /// Whether this function should return an error value if the method call fails  
    /// If true, then the message will be found in a corresponding `#meta_param` field marked with `error`  
    /// #meta: throws;  
    pub throws: bool,

    /// Whether this function should be considered a Destructor, which frees the resources used
    /// #meta: destructor;
    pub is_destructor: bool,

    /// Whether this function should be considered a Constructor for creating new instances
    /// #meta: constructor;
    pub is_constructor: bool,

    /// Whether this value should be represented by a `String` in most languages
    /// #meta_param: value_name;string;
    pub is_string: bool,

    /// Whether this value is a representing a Map or Dictionary
    /// #meta_param: value_name;hashmap;
    pub is_hashmap: bool,

    /// Whether this value is a pointer to a string that will handle the Error Message of this function
    /// #meta_param: value_name;error;
    pub is_error: bool,

    /// Whether this value should be represented by the language's native Duration/TimeSpan type (or otherwise a long)
    /// #meta_param: value_name;duration;
    pub is_duration: bool,

    /// Whether this value should be represented by the language's native DateTime type (or otherwise a long)
    /// #meta_param: value_name;datetime;
    pub is_datetime: bool,

    pub is_output: bool,

    /// Whether this value should be represented by the language's native URI type
    /// #meta_param: value_name;url;
    pub is_url: bool,

    /// Whether this value should be represented by the language's native Timestamp type (or otherwise a long)
    /// #meta_param: value_name;timestamp;
    pub is_timestamp: bool,

    /// If `list` is set, this value is the `len` of the array
    /// #meta_param: value_name;length_for(#param);
    pub length_for: Option<String>,

    /// If `list` is set, this value is the `cap` of the array
    /// #meta_param: value_name;capacity_for(#param);
    pub capacity_for: Option<String>,
}

impl MetaValue {
    /// `true` if the meta object is totally unset for all properties
    pub fn is_empty(&self) -> bool {
        return !self.for_struct
            && !self.is_static
            && !self.is_nullable
            && !self.is_list
            && !self.is_this
            && !self.throws
            && !self.is_destructor
            && !self.is_constructor
            && !self.is_string
            && !self.is_hashmap
            && !self.is_error
            && !self.is_duration
            && !self.is_datetime
            && !self.is_output
            && !self.is_url
            && !self.is_timestamp
            && !self.as_ptr
            && !self.is_void
            && !self.is_async
            && matches!(self.length_for, None)
            && matches!(self.capacity_for, None);
    }
    pub fn new() -> Self {
        MetaValue {
            is_async: false,
            is_void: false,
            is_persistent: false,
            for_struct: false,
            is_list: false,
            is_nullable: false,
            is_static: false,
            throws: false,
            is_destructor: false,
            is_constructor: false,
            is_error: false,
            is_hashmap: false,
            is_string: false,
            is_duration: false,
            is_datetime: false,
            is_output: false,
            is_url: false,
            is_timestamp: false,
            is_this: false,
            as_ptr: false,
            length_for: None,
            capacity_for: None,
        }
    }

    pub fn from_meta_comment_for_param(cmt: &Option<String>, param_name: &str) -> Option<Self> {
        if let Some(c) = cmt {
            // This matcher is of the form `#meta_param: <name>;`, e..g,  #meta_param: encrypted;`
            let match_param_str = format!("{} {};", META_PARAM_TOKEN, param_name);
            let ls: Vec<&str> = c
                .split("\n")
                .filter(|l| l.contains(&match_param_str))
                .collect();

            let pseudo_comment = ls.join("\n");

            let meta_matcher = Regex::new(r"(\w+(?:\(\w+\))?);").unwrap();
            let mut meta = MetaValue::new();
            let mut mm = meta_matcher.captures_iter(&pseudo_comment);
            while let Some(cap) = mm.next() {
                if let Ok(capture) = cap {
                    let mut citer = capture.iter();
                    while let Some(cc) = citer.next() {
                        if let Some(match_value) = cc {
                            let m = match_value.as_str().trim_end_matches(';');
                            meta.modify_from_keyword(m);
                        }
                    }
                }
            }
            if meta.is_empty() {
                None
            } else {
                Some(meta)
            }
        } else {
            None
        }
    }

    /// Takes a meta value, and a meta_keyword and assigns the appropriate meta tag based on the keyword
    fn modify_from_keyword(&mut self, m: &str) {
        match m {
            "async" => self.is_async = true,
            "void" => self.is_void = true,
            "persistent" => self.is_persistent = true,
            "this" => self.is_this = true,
            "for_struct" => self.for_struct = true,
            "list" => self.is_list = true,
            "nullable" => self.is_nullable = true,
            "static" => self.is_static = true,
            "throws" => self.throws = true,
            "destructor" => self.is_destructor = true,
            "constructor" => self.is_constructor = true,
            "string" => self.is_string = true,
            "hashmap" => self.is_hashmap = true,
            "error" => self.is_error = true,
            "duration" => self.is_duration = true,
            "datetime" => self.is_datetime = true,
            "output" => self.is_output = true,
            "url" => self.is_url = true,
            "as_ptr" => self.as_ptr = true,
            "timestamp" => self.is_timestamp = true,
            _ => {
                let compound_matcher = Regex::new(r"(\w+)\((\w+)\)").unwrap();
                let c = compound_matcher.captures(m);
                match c {
                    Ok(cap) => match cap {
                        Some(ccc) => {
                            if ccc.len() == 3 {
                                match ccc.get(1) {
                                    Some(ccc_inner) => match ccc_inner.as_str() {
                                        "length" => {
                                            self.length_for =
                                                Some(ccc.get(2).unwrap().as_str().to_string())
                                        }
                                        "capacity" => {
                                            self.capacity_for =
                                                Some(ccc.get(2).unwrap().as_str().to_string())
                                        }
                                        _ => (),
                                    },
                                    None => {
                                        panic!("invalid inner(item) format in meta")
                                    }
                                }
                            }
                        }
                        None => {}
                    },
                    Err(_) => {}
                };
            }
        }
    }

    pub fn from_meta_comment_dontcare(cmt: &Option<String>) -> Option<Self> {
        if let Some(c) = cmt {
            for s in c.split('\n') {
                if s.contains(META_TOKEN) || s.contains(META_PARAM_TOKEN) {
                    return MetaValue::from_meta_comment(s);
                }
            }
        }
        None
    }

    pub fn from_meta_comment(cmt: &str) -> Option<Self> {
        let meta_matcher = Regex::new(r"(\w+(?:\(\w+\))?);").unwrap();
        let mut meta = MetaValue::new();
        let mut mm = meta_matcher.captures_iter(cmt);
        while let Some(cap) = mm.next() {
            if let Ok(capture) = cap {
                let mut citer = capture.iter();
                while let Some(cc) = citer.next() {
                    if let Some(match_value) = cc {
                        let m = match_value.as_str().trim_end_matches(';');
                        meta.modify_from_keyword(m);
                    }
                }
            }
        }
        if meta.is_empty() {
            None
        } else {
            Some(meta)
        }
    }
}
