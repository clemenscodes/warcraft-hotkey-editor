/// The published `View` contract mirroring [`HelpBodyTextModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpBodyTextView {
    pub text: String,
}

impl ddd::View for HelpBodyTextView {}
