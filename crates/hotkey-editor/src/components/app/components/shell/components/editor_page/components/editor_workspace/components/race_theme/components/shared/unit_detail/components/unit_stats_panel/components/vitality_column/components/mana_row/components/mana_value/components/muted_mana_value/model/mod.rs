use super::view::MutedManaValueView;
use dioxus::prelude::*;

/// The muted mana leaf's input: the shaped display text, built by the dispatcher from
/// the unit's mana pool.
#[derive(Props, Clone, PartialEq)]
pub struct MutedManaValueModel {
    #[props(into)]
    pub text: String,
}

impl From<&MutedManaValueView> for MutedManaValueModel {
    fn from(view: &MutedManaValueView) -> Self {
        let MutedManaValueView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for MutedManaValueModel {
    type View = MutedManaValueView;
}
