use super::view::MutedStatValueView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MutedStatValueModel {
    #[props(into)]
    pub text: String,
}

impl From<&MutedStatValueView> for MutedStatValueModel {
    fn from(view: &MutedStatValueView) -> Self {
        let MutedStatValueView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for MutedStatValueModel {
    type View = MutedStatValueView;
}
