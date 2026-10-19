#[derive(Clone, PartialEq)]
pub struct FooterLinkView {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: Option<&'static str>,
}

impl ddd::Layered for FooterLinkView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FooterLinkView {}
