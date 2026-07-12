/// The published `View` contract mirroring [`ErrorToastContentModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ErrorToastContentView {
    pub title: String,
    pub description: Option<String>,
}

impl ddd::View for ErrorToastContentView {}
