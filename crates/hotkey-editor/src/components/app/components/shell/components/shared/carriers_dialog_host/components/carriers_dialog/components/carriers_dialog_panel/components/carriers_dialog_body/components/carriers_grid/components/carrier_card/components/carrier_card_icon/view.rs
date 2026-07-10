/// The published `View` contract mirroring [`CarrierCardIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarrierCardIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for CarrierCardIconView {}
