use crate::cssom;
use crate::dom::elements::Tag;

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
pub fn get_stylesheet() -> cssom::StyleSheet {
    use cssom::properties::{Color, FontWeight, Measure, MeasureValue, Property};
    use cssom::{CSSRule, Decleration, Selector, StyleSheet};

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
