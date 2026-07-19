use super::view::MutedManaRegenGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MutedManaRegenGainModel {
    #[props(into)]
    pub text: String,
}

impl From<&MutedManaRegenGainView> for MutedManaRegenGainModel {
    fn from(view: &MutedManaRegenGainView) -> Self {
        let MutedManaRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for MutedManaRegenGainModel {
    type View = MutedManaRegenGainView;
}
