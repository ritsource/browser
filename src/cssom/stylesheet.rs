use super::properties;
use crate::dom::elements::Tag;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub struct Decleration(properties::Property);

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
pub struct StyleSheet(HashMap<Selector, CSSRule>);

#[allow(dead_code)]
impl StyleSheet {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[allow(unused_macros)]
macro_rules! hashmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[allow(dead_code)]
pub fn join_fules() {}

#[allow(dead_code)]
pub fn get_test_stylesheet<'a>() -> StyleSheet {
    use properties::{Color, FontWeight, Measure, MeasureValue, Property};

    let rule_2 = CSSRule {
        declerations: vec![
            Decleration(Property::MarginLeft(Measure::Px(MeasureValue::new(10, 0)))),
            Decleration(Property::MarginRight(Measure::Px(MeasureValue::new(10, 0)))),
            Decleration(Property::MarginTop(Measure::Px(MeasureValue::new(0, 0)))),
            Decleration(Property::MarginBottom(Measure::Px(MeasureValue::new(0, 0)))),
        ],
        parent: None,
        ancestor: None,
    };

    let rule_1 = CSSRule {
        declerations: vec![Decleration(Property::Background(Color::new(0, 0, 0, 1)))],
        parent: Some(hashmap! {
            Selector::Tag(Tag::Div) => rule_2
        }),
        ancestor: None,
    };

    // let rule_3 = CSSRule::<'a> {
    //     declerations: vec![Decleration(Property::Color(Color::new(255, 255, 0, 1)))],
    //     parent: None,
    //     ancestor: None,
    // };

    let rule_4 = CSSRule {
        declerations: vec![
            Decleration(Property::Background(Color::new(255, 255, 0, 1))),
            Decleration(Property::Height(Measure::Px(MeasureValue::new(100, 0)))),
            Decleration(Property::Width(Measure::Px(MeasureValue::new(100, 0)))),
        ],
        parent: None,
        ancestor: None,
    };

    let rule_5 = CSSRule {
        declerations: vec![Decleration(Property::FontWeight(FontWeight::Bold))],
        parent: None,
        ancestor: None,
    };

    let rule_0 = CSSRule {
        declerations: vec![Decleration(Property::Color(Color::new(255, 0, 0, 1)))],
        parent: Some(hashmap! {
            Selector::Tag(Tag::Span) => rule_1,
            Selector::Tag(Tag::Footer) => rule_5
        }),
        // parent: None,
        ancestor: None,
    };

    let span = Selector::Tag(Tag::Span);
    let nav = Selector::Tag(Tag::Nav);

    StyleSheet(hashmap! {
        span => rule_0,
        nav => rule_4
    })
}
