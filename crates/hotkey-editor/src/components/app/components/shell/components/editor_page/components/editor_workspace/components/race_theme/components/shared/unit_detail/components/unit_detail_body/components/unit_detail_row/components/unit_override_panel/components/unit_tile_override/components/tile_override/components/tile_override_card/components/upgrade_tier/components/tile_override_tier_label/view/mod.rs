/// The published `View` contract mirroring [`TileOverrideTierLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideTierLabelView {
    pub text: String,
}

impl ddd::View for TileOverrideTierLabelView {}
