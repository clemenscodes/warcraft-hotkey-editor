/// The published `View` contract mirroring [`ObjectIdProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ObjectIdView {
    pub text: String,
}

impl ddd::View for ObjectIdView {}
