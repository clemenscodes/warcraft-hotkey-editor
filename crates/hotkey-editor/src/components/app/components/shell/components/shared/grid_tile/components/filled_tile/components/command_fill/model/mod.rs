use super::view::CommandFillView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandFillModel {
    pub active: bool,
}

impl From<&CommandFillView> for CommandFillModel {
    fn from(view: &CommandFillView) -> Self {
        let CommandFillView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Model for CommandFillModel {
    type View = CommandFillView;
}
