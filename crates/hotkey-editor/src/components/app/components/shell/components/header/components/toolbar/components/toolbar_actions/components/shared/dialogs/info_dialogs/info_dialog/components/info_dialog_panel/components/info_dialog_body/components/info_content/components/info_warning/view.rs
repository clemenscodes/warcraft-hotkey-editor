/// The published `View` contract mirroring [`InfoWarningProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoWarningView {
    pub warning: Option<&'static str>,
}

impl ddd::View for InfoWarningView {}
