#[derive(Clone, PartialEq)]
pub struct ToolbarButtonIconView {
    pub icon: &'static str,
}

impl ddd::Layered for ToolbarButtonIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ToolbarButtonIconView {}
