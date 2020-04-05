use std::collections::HashMap;

#[allow(dead_code)]
pub struct Attributes(HashMap<String, String>);

#[allow(dead_code)]
impl Attributes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from(hmap: HashMap<String, String>) -> Self {
        Self(hmap)
    }

    pub fn from_str() {}

    pub fn map(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Styles();

impl Default for Styles {
    fn default() -> Self {
        Self()
    }
}
