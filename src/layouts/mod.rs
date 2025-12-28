use serde::{Deserialize, Serialize};

mod adv360;
mod glove80;
mod imprint_letters_only_full_bottom_row;
mod sweep;

#[derive(Serialize, Deserialize, Default)]
pub enum KeyboardLayoutType {
    #[serde(rename = "kinesis:adv360")]
    #[default]
    Adv360,
    #[serde(rename = "sweep")]
    Sweep,
    #[serde(rename = "moergo:glove80")]
    Glove80,
    #[serde(rename = "imprint:letters_only_full_bottom_row")]
    ImprintLettersOnlyFullBottomRow,
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
        KeyboardLayoutType::Sweep => sweep::get_layout(),
        KeyboardLayoutType::Glove80 => glove80::get_layout(),
        KeyboardLayoutType::ImprintLettersOnlyFullBottomRow => {
            imprint_letters_only_full_bottom_row::get_layout()
        }
    }
}
