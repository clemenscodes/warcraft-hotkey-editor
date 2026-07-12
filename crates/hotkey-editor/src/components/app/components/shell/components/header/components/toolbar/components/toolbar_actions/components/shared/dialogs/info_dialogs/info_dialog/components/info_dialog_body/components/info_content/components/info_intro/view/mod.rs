/// The published `View` contract mirroring [`InfoIntroModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoIntroView {
    pub intro: &'static str,
}

impl ddd::View for InfoIntroView {}
