use super::properties::Property;
use crate::dom::elements::Tag;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Decleration(Property);

#[allow(dead_code)]
pub enum Selector {
    Tag(Tag),
    Class(String),
    Id(String),
}

#[allow(dead_code)]
pub struct CSSRule {
    declerations: Vec<Decleration>,
    parent: HashMap<Selector, CSSRule>,
    ancestor: HashMap<Selector, CSSRule>,
}

#[allow(dead_code)]
pub struct StyleSheet(HashMap<Selector, CSSRule>);

#[allow(dead_code)]
impl StyleSheet {}
