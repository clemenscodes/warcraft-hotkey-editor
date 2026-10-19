use super::view::MutedManaValueView;
use dioxus::prelude::*;

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

impl ddd::Layered for MutedManaValueModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for MutedManaValueModel {
    type View = MutedManaValueView;
}
