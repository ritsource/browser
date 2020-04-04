use super::node::Node;

#[allow(dead_code)]
pub struct Tree {
    pub root: Node,
}

#[allow(dead_code)]
impl Tree {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn append_child(&mut self, child: Node) {
        self.root.append_child(child);
    }
}
