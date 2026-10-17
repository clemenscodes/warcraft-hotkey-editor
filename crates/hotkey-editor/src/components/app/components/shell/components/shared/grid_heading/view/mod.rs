#[derive(Clone, PartialEq)]
pub struct GridHeadingView {
    pub heading: &'static str,
}

impl ddd::Layered for GridHeadingView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for GridHeadingView {}
