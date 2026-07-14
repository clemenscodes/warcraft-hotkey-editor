#[derive(Clone, PartialEq)]
pub struct ToolbarButtonIconView {
    pub icon: &'static str,
}

impl ddd::View for ToolbarButtonIconView {}
