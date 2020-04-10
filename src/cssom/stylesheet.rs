use super::properties;
use crate::dom::elements::Tag;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub struct Decleration(pub properties::Property);

#[allow(dead_code)]
#[derive(Eq, PartialEq, Hash)]
pub enum Selector {
    Tag(Tag),
    Class(String),
    Id(String),
    Universal,
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub struct CSSRule {
    pub declerations: Vec<Decleration>,
    pub parent: Option<HashMap<Selector, CSSRule>>,
    pub ancestor: Option<HashMap<Selector, CSSRule>>,
}

impl Default for CSSRule {
    fn default() -> Self {
        Self {
            declerations: vec![],
            parent: None,
            ancestor: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub struct StyleSheet(pub HashMap<Selector, CSSRule>);

#[allow(dead_code)]
impl StyleSheet {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[allow(dead_code)]
pub fn join_fules() {}
