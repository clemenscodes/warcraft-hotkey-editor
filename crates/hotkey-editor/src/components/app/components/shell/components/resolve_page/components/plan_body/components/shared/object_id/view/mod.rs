#[derive(Clone, PartialEq)]
pub struct ObjectIdView {
    pub text: String,
}

impl ddd::View for ObjectIdView {}
