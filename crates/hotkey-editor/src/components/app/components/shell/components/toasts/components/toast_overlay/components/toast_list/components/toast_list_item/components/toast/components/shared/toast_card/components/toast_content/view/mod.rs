/// The published `View` contract mirroring [`ToastContentModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToastContentView {
    pub title: String,
    pub description: Option<String>,
}

impl ddd::View for ToastContentView {}
