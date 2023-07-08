pub struct Context {
    pub indent: usize,
    pub keymap: bool,
}

impl Context {
    pub fn with_indent(&self, indent: usize) -> Self {
        Self {
            indent,
            keymap: self.keymap,
        }
    }

    pub fn inc(&self, increment: usize) -> Self {
        Self {
            indent: self.indent + increment,
            keymap: self.keymap,
        }
    }
}
