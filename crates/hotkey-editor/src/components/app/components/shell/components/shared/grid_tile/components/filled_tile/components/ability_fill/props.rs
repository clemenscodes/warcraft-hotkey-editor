use super::view::AbilityFillView;
use dioxus::prelude::*;

/// The ability fill draws only when the occupant is an ordinary ability (a selected
/// tile keeps the ability background too). A command occupant leaves `active` false and
/// draws `CommandFill` instead, so this fill early-returns.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityFillProps {
    pub active: bool,
}

impl From<&AbilityFillView> for AbilityFillProps {
    fn from(view: &AbilityFillView) -> Self {
        let AbilityFillView { active } = view.clone();
        Self { active }
    }
}

impl ddd::Props for AbilityFillProps {
    type View = AbilityFillView;
}
