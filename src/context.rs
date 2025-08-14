use crate::config::Config;

pub struct Context<'a> {
    pub indent: usize,
    pub keymap: bool,
    pub bindings: bool,
    pub config: &'a Config,
}

impl Context<'_> {
    pub fn inc(&self, increment: usize) -> Self {
        Self { indent: self.indent + increment, ..*self }
    }

    pub fn dec(&self, decrement: usize) -> Self {
        Self { indent: self.indent - decrement, ..*self }
    }

    pub fn keymap(&self) -> Self {
        Self { keymap: true, ..*self }
    }

    pub fn bindings(&self) -> Self {
        Self { bindings: true, ..*self }
    }

    // If a node named 'bindings' has a parent node named 'keymap' then we've
    // encountered a Zephyr keymap that will be handled as a special case by the
    // printer.
    pub fn has_zephyr_syntax(&self) -> bool {
        self.bindings && self.keymap
    }
}
