#[derive(Clone, PartialEq)]
pub struct InfoIntroView {
    pub intro: &'static str,
}

impl ddd::Layered for InfoIntroView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for InfoIntroView {}
