use std::collections::HashMap;

#[allow(dead_code)]
pub struct Attributes(HashMap<String, String>);

#[allow(dead_code)]
impl Attributes {
    pub fn from(hmap: HashMap<String, String>) -> Self {
        Self(hmap)
    }

    pub fn from_str() {}
}

impl Default for Attributes {
    fn default() -> Self {
        let x: HashMap<String, String> = HashMap::new();
        Self(x)
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
