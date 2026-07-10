/// The published `View` contract mirroring [`RegenQualifierProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RegenQualifierView {
    pub text: Option<&'static str>,
}

impl ddd::View for RegenQualifierView {}
