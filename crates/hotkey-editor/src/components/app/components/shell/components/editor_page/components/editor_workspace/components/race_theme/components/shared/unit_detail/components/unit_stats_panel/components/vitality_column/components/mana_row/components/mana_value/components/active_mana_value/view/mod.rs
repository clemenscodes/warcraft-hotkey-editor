/// The published `View` contract mirroring [`ActiveManaValueModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveManaValueView {
    pub text: String,
}

impl ddd::View for ActiveManaValueView {}
