/// The published `View` contract mirroring [`CarrierCardNameProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CarrierCardNameView {
    pub text: String,
}

impl ddd::View for CarrierCardNameView {}
