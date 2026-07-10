use dioxus::prelude::*;

/// The command fill draws only when the occupant is a built-in command; every other
/// occupant leaves `active` false and draws `AbilityFill` instead, so this fill
/// early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct CommandFillProps {
    pub active: bool,
}
