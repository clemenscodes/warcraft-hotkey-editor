#[derive(Clone, PartialEq)]
pub struct FooterLinkIconView {
    pub icon: Option<&'static str>,
}

impl ddd::Layered for FooterLinkIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FooterLinkIconView {}
