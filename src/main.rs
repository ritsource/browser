mod cssom;
mod dom;

// use dom::document::Document;
// use dom::elements::Elem;
// use dom::node::Node;

fn try_join() {}

fn main() {
    try_join();

    // let mut dom = Document::new();

    // let html = String::from("<a href=\"https://wikipedia.com\">Wikipedia</a>");
    // let elem = naive_parse(html);

    // dom.add_root(Node::from_elem(elem));
}

// fn naive_parse(_: String) -> Elem {
//     use dom::elements::Attributes;
//     use std::collections::HashMap;

//     let (tag_name, attr_map) = {
//         let mut hm: HashMap<String, String> = HashMap::new();
//         hm.insert("href".to_string(), "https://wikipedia.com".to_string());

//         ("a", hm)
//     };

//     Elem::from(tag_name).with_attributes(Attributes::from(attr_map))
// }
