use super::view::CommandFillView;
use dioxus::prelude::*;

/// The command fill draws only when the occupant is a built-in command; every other
/// occupant leaves `active` false and draws `AbilityFill` instead, so this fill
/// early-returns.
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
