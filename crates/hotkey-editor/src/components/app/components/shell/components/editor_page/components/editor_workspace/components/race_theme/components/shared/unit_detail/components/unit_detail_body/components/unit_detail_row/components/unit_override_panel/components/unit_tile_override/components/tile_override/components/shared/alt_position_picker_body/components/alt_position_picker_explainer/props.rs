use super::view::AltPositionPickerExplainerView;
use dioxus::prelude::*;

/// The instruction text shown above the picker grid.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerExplainerProps {
    #[props(into)]
    pub text: String,
}

impl From<&AltPositionPickerExplainerView> for AltPositionPickerExplainerProps {
    fn from(view: &AltPositionPickerExplainerView) -> Self {
        let AltPositionPickerExplainerView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for AltPositionPickerExplainerProps {
    type View = AltPositionPickerExplainerView;
}
