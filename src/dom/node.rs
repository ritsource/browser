use crate::elements::elem::Elem;

#[allow(dead_code)]
pub struct Node {
    pub element: Elem,
    pub children: Vec<Node>,
}

#[allow(dead_code)]
impl Node {
    pub fn new() -> Self {
        Self {
            element: Elem::new(),
            children: vec![],
        }
    }

    pub fn from(elem: Elem) -> Self {
        Self {
            element: elem,
            children: vec![],
        }
    }

    pub fn from_tag_name(tag_name: &str) -> Self {
        Self::from(Elem::from(tag_name))
    }

    // pub fn get_element(&self) -> Elem {
    //     self.element
    // }

    pub fn update_element(&mut self, elem: Elem) {
        self.element = elem;
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
