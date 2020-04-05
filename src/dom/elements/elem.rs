use super::elemtype::ElemType;
use super::props::{Attributes, Styles};

#[allow(dead_code)]
pub struct Elem {
    pub tag: ElemType,
    pub attributes: Attributes,
    pub styles: Styles,
}

#[allow(dead_code)]
impl Elem {
    pub fn new() -> Self {
        Self::from("")
    }

    pub fn from(tagname: &str) -> Self {
        Self {
            attributes: Attributes::default(),
            styles: Styles::default(),
            tag: ElemType::from(tagname),
        }
    }

    pub fn with_attributes(mut self, attrs: Attributes) -> Self {
        self.attributes = attrs;
        self
    }

    pub fn with_styles(mut self, styles: Styles) -> Self {
        self.styles = styles;
        self
    }
}
