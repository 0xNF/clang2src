use fancy_regex::Regex;
use serde::Serialize;

pub const META_TOKEN: &str = "#meta";
pub const META_PARAM_TOKEN: &str = "#meta_param";

#[derive(Serialize, Clone)]
pub struct MetaValue {
    pub is_persistent: bool,
    pub for_struct: bool,
    pub is_static: bool,
    pub is_nullable: bool,
    pub is_list: bool,
    pub is_this: bool,
    pub throws: bool,
    pub is_destructor: bool,
    pub is_constructor: bool,
    pub is_string: bool,
    pub is_hashmap: bool,
    pub is_error: bool,
    pub is_duration: bool,
    pub is_datetime: bool,
    pub is_output: bool,
    pub is_url: bool,
    pub is_timestamp: bool,
    pub length_for: Option<String>,
    pub capacity_for: Option<String>,
}

impl MetaValue {
    pub fn new() -> Self {
        MetaValue {
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
            length_for: None,
            capacity_for: None,
        }
    }

    pub fn from_meta_comment_dontcare(cmt: &Option<String>) -> Self {
        if let Some(c) = cmt {
            for s in c.split('\n') {
                if s.contains("#meta:") {
                    return MetaValue::from_meta_comment(s);
                }
            }
        }
        MetaValue::new()
    }

    pub fn from_meta_comment(cmt: &str) -> Self {
        let compound_matcher = Regex::new(r"(\w+)\((\w+)\)").unwrap();
        let meta_matcher = Regex::new(r"(\w+(?:\(\w+\))?);").unwrap(); //Regex::new(r"(\w+);").unwrap();
        let mut meta = MetaValue::new();
        let mut mm = meta_matcher.captures_iter(cmt);
        while let Some(cap) = mm.next() {
            if let Ok(capture) = cap {
                let mut citer = capture.iter();
                while let Some(cc) = citer.next() {
                    if let Some(match_value) = cc {
                        let m = match_value.as_str().trim_end_matches(';');
                        match m {
                            "persistent" => meta.is_persistent = true,
                            "this" => meta.is_this = true,
                            "for_struct" => meta.for_struct = true,
                            "list" => meta.is_list = true,
                            "nullable" => meta.is_nullable = true,
                            "static" => meta.is_static = true,
                            "throws" => meta.throws = true,
                            "free" => meta.is_destructor = true,
                            "constructor" => meta.is_constructor = true,
                            "string" => meta.is_string = true,
                            "hashmap" => meta.is_hashmap = true,
                            "error" => meta.is_error = true,
                            "duration" => meta.is_duration = true,
                            "datetime" => meta.is_datetime = true,
                            "output" => meta.is_output = true,
                            "url" => meta.is_url = true,
                            "timestamp" => meta.is_timestamp = true,
                            _ => {
                                let c = compound_matcher.captures(m);
                                match c {
                                    Ok(cap) => match cap {
                                        Some(ccc) => {
                                            if ccc.len() == 3 {
                                                match ccc.get(1) {
                                                    Some(ccc_inner) => match ccc_inner.as_str() {
                                                        "length" => {
                                                            meta.length_for = Some(
                                                                ccc.get(2)
                                                                    .unwrap()
                                                                    .as_str()
                                                                    .to_string(),
                                                            )
                                                        }
                                                        "capacity" => {
                                                            meta.capacity_for = Some(
                                                                ccc.get(2)
                                                                    .unwrap()
                                                                    .as_str()
                                                                    .to_string(),
                                                            )
                                                        }
                                                        _ => (),
                                                    },
                                                    None => {
                                                        panic!("invalid inner(item) format in meta")
                                                    }
                                                }
                                            }
                                        }
                                        None => break,
                                    },
                                    Err(_) => break,
                                };
                            }
                        }
                    }
                }
            }
        }
        meta
    }
}
