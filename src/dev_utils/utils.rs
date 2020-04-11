use crate::cssom;
use crate::dom;

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
pub fn get_document() -> dom::Document {
    use dom::elements::Elem;
    use dom::{Document, Node};

    let node = |tn: &str, chs: Vec<Node>| Node::from(Elem::from(tn)).with_children(chs);

    Document::new().with_root(node(
        "html",
        vec![
            node("head", vec![]),
            node(
                "body",
                vec![
                    node("nav", vec![]),
                    node(
                        "div",
                        vec![
                            node("h1", vec![Node::text("Some 1")]),
                            node(
                                "p",
                                vec![
                                    Node::text("This is "),
                                    node("span", vec![Node::text("p tag")]),
                                    Node::text("under div"),
                                ],
                            ),
                            node("h1", vec![Node::text("Some 2")]),
                            node("p", vec![Node::text("another p tag")]),
                        ],
                    ),
                    node("footer", vec![]),
                ],
            ),
        ],
    ))
}

#[allow(dead_code)]
pub fn get_stylesheet() -> cssom::StyleSheet {
    use cssom::properties::{Color, FontWeight, Measure, MeasureValue, Property};
    use cssom::{CSSRule, Decleration, Selector, StyleSheet};
    use dom::elements::Tag;

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
