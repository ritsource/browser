#[allow(dead_code)]
pub struct Node {
    data: Option<()>,
    children: Vec<Node>,
}

#[allow(dead_code)]
impl Node {
    pub fn new() -> Self {
        Self {
            data: None,
            children: vec![],
        }
    }

    pub fn get_data(&self) -> Option<()> {
        self.data
    }

    pub fn update_data(&mut self, data: Option<()>) {
        self.data = data;
    }

    pub fn get_child(&self, index: usize) -> &Node {
        &self.children[index]
    }

    pub fn append_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, index: usize, child: Node) {
        self.children.insert(index, child);
    }

    pub fn remove_child(&mut self, index: usize) -> Node {
        self.children.remove(index)
    }

    pub fn children_count(&self) -> usize {
        self.children.len()
    }
}
