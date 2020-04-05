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
pub struct Selection {
    declerations: Vec<Decleration>,
    direct_parents: Vec<Selection>,
    parents: Vec<Selection>,
}

#[allow(dead_code)]
pub struct StyleSheet(HashMap<Selector, Selection>);

#[allow(dead_code)]
impl StyleSheet {}
