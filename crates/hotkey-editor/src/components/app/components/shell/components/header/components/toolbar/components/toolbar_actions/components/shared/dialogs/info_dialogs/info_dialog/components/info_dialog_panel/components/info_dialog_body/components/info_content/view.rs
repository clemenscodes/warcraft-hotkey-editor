/// The published `View` contract mirroring [`InfoContentProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoContentView {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
}

impl ddd::View for InfoContentView {}
