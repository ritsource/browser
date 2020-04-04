mod dom;

use dom::node::Node;
use dom::tree::Tree;

fn main() {
    let mut parent = Node::new();
    {
        parent.append_child(Node::new());
    }
    println!("Length - {}", parent.children_count());
}
