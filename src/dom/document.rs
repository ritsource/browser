use super::node::Node;

#[allow(dead_code)]
pub struct Document {
    pub root: Option<Node>,
}

#[allow(dead_code)]
impl Document {
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn with_root(mut self, root: Node) -> Self {
        self.root = Some(root);
        self
    }

    pub fn add_root(&mut self, root: Node) {
        self.root = Some(root);
    }

    pub fn root(&mut self) -> &mut Option<Node> {
        &mut self.root
    }
}
