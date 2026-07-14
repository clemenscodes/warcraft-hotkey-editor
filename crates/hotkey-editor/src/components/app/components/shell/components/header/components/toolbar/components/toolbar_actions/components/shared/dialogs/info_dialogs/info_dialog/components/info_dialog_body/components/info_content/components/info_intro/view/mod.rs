#[derive(Clone, PartialEq)]
pub struct InfoIntroView {
    pub intro: &'static str,
}

impl ddd::View for InfoIntroView {}
