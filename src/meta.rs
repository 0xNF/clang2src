use std::str::FromStr;

use fancy_regex::Regex;
use serde::Serialize;

pub const META_TOKEN: &str = "#meta:";
pub const META_PARAM_TOKEN: &str = "#meta_param:";

#[derive(Debug, Serialize, Clone)]
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

    pub fn from_meta_comment_for_param(cmt: &Option<String>, param_name: &str) -> Self {
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
            meta
        } else {
            MetaValue::new()
        }
    }

    /// Takes a meta value, and a meta_keyword and assigns the appropriate meta tag based on the keyword
    fn modify_from_keyword(&mut self, m: &str) {
        match m {
            "persistent" => self.is_persistent = true,
            "this" => self.is_this = true,
            "for_struct" => self.for_struct = true,
            "list" => self.is_list = true,
            "nullable" => self.is_nullable = true,
            "static" => self.is_static = true,
            "throws" => self.throws = true,
            "free" => self.is_destructor = true,
            "constructor" => self.is_constructor = true,
            "string" => self.is_string = true,
            "hashmap" => self.is_hashmap = true,
            "error" => self.is_error = true,
            "duration" => self.is_duration = true,
            "datetime" => self.is_datetime = true,
            "output" => self.is_output = true,
            "url" => self.is_url = true,
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

    pub fn from_meta_comment_dontcare(cmt: &Option<String>) -> Self {
        if let Some(c) = cmt {
            for s in c.split('\n') {
                if s.contains(META_TOKEN) || s.contains(META_PARAM_TOKEN) {
                    return MetaValue::from_meta_comment(s);
                }
            }
        }
        MetaValue::new()
    }

    pub fn from_meta_comment(cmt: &str) -> Self {
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
        meta
    }
}
