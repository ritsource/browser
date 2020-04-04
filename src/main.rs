mod dom;
mod elements;

use dom::node::Node;
use dom::tree::Tree;
use elements::elem::Elem;

fn main() {
    let mut dom = Tree::new();

    let html = String::from("<a href=\"https://wikipedia.com\">Wikipedia</a>");
    let elem = naive_parse(html);

    dom.append_child(Node::from(elem));
}

fn naive_parse(_: String) -> Elem {
    use elements::props::Attributes;
    use std::collections::HashMap;

    let (tag_name, attr_map) = {
        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert("href".to_string(), "https://wikipedia.com".to_string());

        ("a", hm)
    };

    Elem::from(tag_name).with_attributes(Attributes::from(attr_map))
}
