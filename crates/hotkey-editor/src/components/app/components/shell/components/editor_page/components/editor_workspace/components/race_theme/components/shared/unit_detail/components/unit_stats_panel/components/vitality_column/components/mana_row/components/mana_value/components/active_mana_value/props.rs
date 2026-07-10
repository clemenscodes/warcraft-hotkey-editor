use dioxus::prelude::*;

/// The active mana leaf's input: the shaped display text, built by the dispatcher from
/// the unit's mana pool.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveManaValueProps {
    #[props(into)]
    pub text: String,
}
