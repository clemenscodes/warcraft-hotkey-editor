/// The published `View` contract mirroring [`FooterLinkProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FooterLinkView {
    pub label: &'static str,
    pub href: &'static str,
    pub icon: Option<&'static str>,
}

impl ddd::View for FooterLinkView {}
