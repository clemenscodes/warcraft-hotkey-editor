/// The published `View` contract mirroring [`InfoWarningModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InfoWarningView {
    pub warning: Option<&'static str>,
}

impl ddd::View for InfoWarningView {}
