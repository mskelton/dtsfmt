use serde::{Deserialize, Serialize};

mod adv360;

#[derive(Serialize, Deserialize)]
pub enum KeyboardLayoutType {
    #[serde(rename = "kinesis:adv360")]
    Adv360,
}

pub struct KeyboardLayout {
    pub bindings: Vec<usize>,
    pub row_count: usize,
}

impl KeyboardLayout {
    pub fn row_size(&self) -> usize {
        self.bindings.len() / self.row_count
    }
}

pub fn get_layout(layout_type: &KeyboardLayoutType) -> KeyboardLayout {
    match layout_type {
        KeyboardLayoutType::Adv360 => adv360::get_layout(),
    }
}
