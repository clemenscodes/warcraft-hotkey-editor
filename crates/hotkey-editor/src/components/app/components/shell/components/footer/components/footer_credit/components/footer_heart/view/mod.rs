#[derive(Clone, PartialEq)]
pub struct FooterHeartView {
    pub svg: &'static str,
}

impl ddd::Layered for FooterHeartView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FooterHeartView {}
