#[derive(Clone, PartialEq)]
pub struct SystemHotkeysBreadcrumbsTriggerLabelView {
    pub text: String,
}

impl ddd::Layered for SystemHotkeysBreadcrumbsTriggerLabelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemHotkeysBreadcrumbsTriggerLabelView {}
