#[derive(Clone, PartialEq)]
pub struct MutedStatValueView {
    pub text: String,
}

impl ddd::View for MutedStatValueView {}
