use std::collections::BTreeMap;
use std::default::Default;

pub enum PermAttr {  
    Num(i32),
    Str(String),
    Pos {x: i32, y: i32},
    Color(u8, u8, u8)
}

pub struct PermAttrs {
    attrs: BTreeMap<String, PermAttr>
}

// Why use T if not generic?
impl<const T: usize> From<[(String, PermAttr); T]> for PermAttrs {
    fn from(v: [(String, PermAttr); T]) -> Self {
        PermAttrs {
            attrs: v.into()
        }
    }
}

impl Default for PermAttrs {
    fn default() -> Self {
        Self { 
            attrs: Default::default() 
        }
    }
}

impl PermAttrs {
    pub fn get_attr(&self, attr: &str) -> Option<&PermAttr> {
        self.attrs.get(attr)
    }
}
