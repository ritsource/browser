// use std::collections::HashMap;

use crate::dom::elements::Tag;

use super::declaration::Decleration;

#[allow(dead_code)]
pub struct StyleSheet(Vec<CssRule>);

#[allow(dead_code)]
pub struct CssRule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Decleration>,
}

#[allow(dead_code)]
pub enum Selector {
    Tag(Tag),
    Class(String),
    Id(String),
}

#[allow(dead_code)]
impl StyleSheet {}
