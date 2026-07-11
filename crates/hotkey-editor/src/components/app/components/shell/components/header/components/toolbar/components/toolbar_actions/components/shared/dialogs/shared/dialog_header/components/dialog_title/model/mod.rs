use super::view::DialogTitleView;
use dioxus::prelude::*;

/// The title's only input: the heading text.
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleModel {
    #[props(into)]
    pub title: String,
}

impl From<&DialogTitleView> for DialogTitleModel {
    fn from(view: &DialogTitleView) -> Self {
        let DialogTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Model for DialogTitleModel {
    type View = DialogTitleView;
}
