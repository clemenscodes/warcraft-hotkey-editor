/// The published `View` contract mirroring [`FooterDisclaimerProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FooterDisclaimerView {
    pub text: &'static str,
}

impl ddd::View for FooterDisclaimerView {}
