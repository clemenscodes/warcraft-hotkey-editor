/// The published `View` contract mirroring [`FooterLinkIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FooterLinkIconView {
    pub icon: Option<&'static str>,
}

impl ddd::View for FooterLinkIconView {}
