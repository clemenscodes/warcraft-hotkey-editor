/// The published `View` contract mirroring [`SystemSlotLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemSlotLabelView {
    pub text: String,
}

impl ddd::View for SystemSlotLabelView {}
