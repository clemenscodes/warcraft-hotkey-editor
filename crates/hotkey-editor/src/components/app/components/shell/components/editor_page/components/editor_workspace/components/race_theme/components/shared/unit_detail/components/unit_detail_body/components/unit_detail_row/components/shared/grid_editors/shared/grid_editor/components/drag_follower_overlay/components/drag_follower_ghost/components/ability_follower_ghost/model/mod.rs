use super::super::super::presentation::FollowerPresentation;
use super::view::AbilityFollowerGhostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AbilityFollowerGhostModel {
    pub presentation: FollowerPresentation,
}

impl From<&AbilityFollowerGhostView> for AbilityFollowerGhostModel {
    fn from(view: &AbilityFollowerGhostView) -> Self {
        let AbilityFollowerGhostView { presentation } = view.clone();
        Self { presentation }
    }
}

impl ddd::Model for AbilityFollowerGhostModel {
    type View = AbilityFollowerGhostView;
}
