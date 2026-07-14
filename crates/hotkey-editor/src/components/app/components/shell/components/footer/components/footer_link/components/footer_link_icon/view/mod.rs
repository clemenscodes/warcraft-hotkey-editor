#[derive(Clone, PartialEq)]
pub struct FooterLinkIconView {
    pub icon: Option<&'static str>,
}

impl ddd::View for FooterLinkIconView {}
