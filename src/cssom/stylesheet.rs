use super::declaration::Decleration;
use crate::dom::elements::Tag;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct StyleSheet(HashMap<Selector, Decleration>);

#[allow(dead_code)]
pub enum Selector {
    Tag(Tag),
    Class(String),
    Id(String),
}

#[allow(dead_code)]
impl StyleSheet {}
