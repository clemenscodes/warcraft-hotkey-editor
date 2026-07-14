use super::view::MatchupValueView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MatchupValueModel {
    pub multiplier: f32,
}

impl From<&MatchupValueView> for MatchupValueModel {
    fn from(view: &MatchupValueView) -> Self {
        let MatchupValueView { multiplier } = view.clone();
        Self { multiplier }
    }
}

impl ddd::Model for MatchupValueModel {
    type View = MatchupValueView;
}
