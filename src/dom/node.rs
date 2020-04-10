use super::elements::Elem;

#[allow(dead_code)]
pub struct Node {
    pub data: NodeData,
    pub children: Vec<Node>,
}

pub enum NodeData {
    Elem(Elem),
    Text(String),
}

#[allow(dead_code)]
impl Node {
    pub fn from(elem: Elem) -> Self {
        Self {
            data: NodeData::Elem(elem),
            children: vec![],
        }
    }

    pub fn text(text: &str) -> Self {
        Self {
            data: NodeData::Text(text.to_string()),
            children: vec![],
        }
    }

    pub fn with_children(mut self, children: Vec<Node>) -> Self {
        self.children = children;
        self
    }

    pub fn get_data(&mut self) -> &mut NodeData {
        &mut self.data
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
