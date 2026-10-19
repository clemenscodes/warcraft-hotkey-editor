#[derive(Clone, PartialEq)]
pub struct PreviewGridHeadingView {
    pub heading: &'static str,
}

impl ddd::Layered for PreviewGridHeadingView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PreviewGridHeadingView {}
