use dioxus::prelude::*;

/// The instruction text shown above the picker grid.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerExplainerProps {
    #[props(into)]
    pub text: String,
}
