#[derive(Clone, PartialEq)]
pub struct HotkeyOverrideEmptyView {
    pub message: String,
}

impl ddd::Layered for HotkeyOverrideEmptyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HotkeyOverrideEmptyView {}
