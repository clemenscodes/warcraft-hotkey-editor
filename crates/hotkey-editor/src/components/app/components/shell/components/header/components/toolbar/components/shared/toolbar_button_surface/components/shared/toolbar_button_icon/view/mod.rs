/// The published `View` contract mirroring [`ToolbarButtonIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToolbarButtonIconView {
    pub icon: &'static str,
}

impl ddd::View for ToolbarButtonIconView {}
