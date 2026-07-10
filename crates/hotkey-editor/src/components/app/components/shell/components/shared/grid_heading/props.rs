use super::view::GridHeadingView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridHeadingProps {
    pub heading: &'static str,
}

impl From<&GridHeadingView> for GridHeadingProps {
    fn from(view: &GridHeadingView) -> Self {
        let GridHeadingView { heading } = view.clone();
        Self { heading }
    }
}

impl ddd::Props for GridHeadingProps {
    type View = GridHeadingView;
}
