use super::view::DialogTitleView;
use dioxus::prelude::*;

/// The title's only input: the heading text.
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    #[props(into)]
    pub title: String,
}

impl From<&DialogTitleView> for DialogTitleProps {
    fn from(view: &DialogTitleView) -> Self {
        let DialogTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Props for DialogTitleProps {
    type View = DialogTitleView;
}
