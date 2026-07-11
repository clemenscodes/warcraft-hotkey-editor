/// The published `View` contract mirroring [`InfoContentModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoContentView {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
}

impl ddd::View for InfoContentView {}
