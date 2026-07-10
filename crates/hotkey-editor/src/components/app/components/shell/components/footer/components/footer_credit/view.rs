/// The published `View` contract mirroring [`FooterCreditProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FooterCreditView {
    pub lead: &'static str,
    pub tail: &'static str,
    pub heart: &'static str,
}

impl ddd::View for FooterCreditView {}
