#[derive(Clone, PartialEq)]
pub struct ResolvePageView {
    pub entry: Option<String>,
}

impl ddd::Layered for ResolvePageView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ResolvePageView {}
