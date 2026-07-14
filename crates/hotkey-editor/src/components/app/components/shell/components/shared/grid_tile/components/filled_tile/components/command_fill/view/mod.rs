#[derive(Clone, PartialEq)]
pub struct CommandFillView {
    pub active: bool,
}

impl ddd::View for CommandFillView {}
