#[derive(Clone, PartialEq)]
pub struct HotkeyAltPositionPickerExplainerView {
    pub text: String,
}

impl ddd::Layered for HotkeyAltPositionPickerExplainerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HotkeyAltPositionPickerExplainerView {}
