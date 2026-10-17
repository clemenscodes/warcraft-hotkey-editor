#[derive(Clone, PartialEq)]
pub struct DialogTitleView {
    pub title: String,
}

impl ddd::Layered for DialogTitleView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DialogTitleView {}
