#[derive(Clone, PartialEq)]
pub struct SystemHotkeysSectionIntroView {
    pub text: String,
}

impl ddd::Layered for SystemHotkeysSectionIntroView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysSectionIntroView {}
