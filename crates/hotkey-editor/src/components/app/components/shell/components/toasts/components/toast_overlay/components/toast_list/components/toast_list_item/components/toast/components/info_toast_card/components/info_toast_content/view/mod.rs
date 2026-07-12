/// The published `View` contract mirroring [`InfoToastContentModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoToastContentView {
    pub title: String,
    pub description: Option<String>,
}

impl ddd::View for InfoToastContentView {}
