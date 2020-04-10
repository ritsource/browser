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
pub struct CSSRule<'a> {
    pub declerations: Vec<Decleration>,
    pub parent: Option<HashMap<Selector, &'a CSSRule<'a>>>,
    pub ancestor: Option<HashMap<Selector, &'a CSSRule<'a>>>,
}

#[allow(dead_code)]
// impl<'a> CSSRule<'a> {
//     pub fn from(
//         declerations: Vec<Decleration>,
//         parent: Option<HashMap<Selector, &'a CSSRule<'a>>>,
//         ancestor: Option<HashMap<Selector, &'a CSSRule<'a>>>,
//     ) -> &'a Self {
//         &Self::<'a> {
//             declerations,
//             parent,
//             ancestor,
//         }
//     }
// }

impl Default for CSSRule<'_> {
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
pub struct StyleSheet<'a>(HashMap<Selector, CSSRule<'a>>);

#[allow(dead_code)]
impl<'a> StyleSheet<'a> {
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
pub fn get_test_stylesheet<'a>() -> StyleSheet<'a> {
    let span = Selector::Tag(Tag::Span);
    let p = Selector::Tag(Tag::P);
    let div = Selector::Tag(Tag::Div);
    let nav = Selector::Tag(Tag::Nav);

    use properties::{Color, Measure, MeasureValue, Property};

    // let rule_1 = CSSRule::from(
    //     vec![Decleration(Property::Background(Color::new(0, 0, 0, 1)))],
    //     None,
    //     None,
    // );

    let rule_2 = CSSRule::<'a> {
        declerations: vec![
            Decleration(Property::MarginLeft(Measure::Px(MeasureValue::new(10, 0)))),
            Decleration(Property::MarginRight(Measure::Px(MeasureValue::new(10, 0)))),
            Decleration(Property::MarginTop(Measure::Px(MeasureValue::new(0, 0)))),
            Decleration(Property::MarginBottom(Measure::Px(MeasureValue::new(0, 0)))),
        ],
        parent: None,
        ancestor: None,
    };

    // let rule_3 = CSSRule::<'a> {
    //     declerations: vec![Decleration(Property::Color(Color::new(255, 255, 0, 1)))],
    //     parent: None,
    //     ancestor: None,
    // };

    // let x: &'a CSSRule = &rule_1;

    let rule_0 = CSSRule::<'a> {
        declerations: vec![Decleration(Property::Color(Color::new(255, 0, 0, 1)))],
        // parent: Some(hashmap! {Selector::Tag(Tag::Span) => CSSRule::from(
        //     vec![Decleration(Property::Background(Color::new(0, 0, 0, 1)))],
        //     None,
        //     None,
        // )}),
        parent: None,
        ancestor: None,
    };

    let rule_4 = CSSRule::<'a> {
        declerations: vec![
            Decleration(Property::Background(Color::new(255, 255, 0, 1))),
            Decleration(Property::Height(Measure::Px(MeasureValue::new(100, 0)))),
            Decleration(Property::Width(Measure::Px(MeasureValue::new(100, 0)))),
        ],
        parent: None,
        ancestor: None,
    };

    StyleSheet(hashmap! {
        span => rule_0,
        nav => rule_4
    })
}
