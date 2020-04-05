use std::collections::HashMap;

#[allow(dead_code)]
pub struct Styles(HashMap<String, String>);

#[allow(dead_code)]
impl Styles {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn as_mut_map(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

impl Default for Styles {
    fn default() -> Self {
        Self::new()
    }
}
