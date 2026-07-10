/// The published `View` contract mirroring [`ControlPlainIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ControlPlainIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::View for ControlPlainIconView {}
