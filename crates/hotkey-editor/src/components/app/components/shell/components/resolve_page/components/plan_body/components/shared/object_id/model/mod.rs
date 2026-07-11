use super::view::ObjectIdView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ObjectIdModel {
    #[props(into)]
    pub text: String,
}

impl From<&ObjectIdView> for ObjectIdModel {
    fn from(view: &ObjectIdView) -> Self {
        let ObjectIdView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ObjectIdModel {
    type View = ObjectIdView;
}
