#[derive(Clone, PartialEq)]
pub struct CollisionsPageView {
    pub kind: Option<String>,
    pub entry: Option<String>,
}

impl ddd::Layered for CollisionsPageView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CollisionsPageView {}
