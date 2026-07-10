/// The published `View` contract mirroring [`WarningToastContentProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct WarningToastContentView {
    pub title: String,
    pub description: Option<String>,
}

impl ddd::View for WarningToastContentView {}
