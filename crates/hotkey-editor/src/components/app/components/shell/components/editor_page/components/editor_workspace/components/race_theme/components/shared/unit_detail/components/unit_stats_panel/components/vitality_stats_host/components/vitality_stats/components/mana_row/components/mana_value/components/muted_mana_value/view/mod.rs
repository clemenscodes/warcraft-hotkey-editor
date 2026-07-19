#[derive(Clone, PartialEq)]
pub struct MutedManaValueView {
    pub text: String,
}

impl ddd::View for MutedManaValueView {}
