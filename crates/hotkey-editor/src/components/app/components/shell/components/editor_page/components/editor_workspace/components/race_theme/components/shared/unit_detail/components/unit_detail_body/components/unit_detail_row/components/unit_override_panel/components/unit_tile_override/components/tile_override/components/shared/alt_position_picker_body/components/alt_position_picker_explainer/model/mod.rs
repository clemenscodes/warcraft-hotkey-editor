use super::view::AltPositionPickerExplainerView;
use dioxus::prelude::*;

/// The instruction text shown above the picker grid.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerExplainerModel {
    #[props(into)]
    pub text: String,
}

impl From<&AltPositionPickerExplainerView> for AltPositionPickerExplainerModel {
    fn from(view: &AltPositionPickerExplainerView) -> Self {
        let AltPositionPickerExplainerView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AltPositionPickerExplainerModel {
    type View = AltPositionPickerExplainerView;
}
