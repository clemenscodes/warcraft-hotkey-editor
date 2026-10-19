#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerCaretView {
    pub is_open: bool,
}

impl ddd::Layered for SystemHotkeysBreadcrumbsTriggerCaretView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysBreadcrumbsTriggerCaretView {}
