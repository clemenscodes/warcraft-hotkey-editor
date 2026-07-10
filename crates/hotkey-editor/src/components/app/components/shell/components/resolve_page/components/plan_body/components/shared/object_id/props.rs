use super::view::ObjectIdView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ObjectIdProps {
    #[props(into)]
    pub text: String,
}

impl From<&ObjectIdView> for ObjectIdProps {
    fn from(view: &ObjectIdView) -> Self {
        let ObjectIdView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for ObjectIdProps {
    type View = ObjectIdView;
}
