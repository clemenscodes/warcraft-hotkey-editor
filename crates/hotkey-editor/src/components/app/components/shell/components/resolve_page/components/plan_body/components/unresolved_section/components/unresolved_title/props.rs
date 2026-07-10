use super::view::UnresolvedTitleView;
use dioxus::prelude::*;

/// The unresolved-abilities section heading text.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedTitleProps {
    pub text: &'static str,
}

impl From<&UnresolvedTitleView> for UnresolvedTitleProps {
    fn from(view: &UnresolvedTitleView) -> Self {
        let UnresolvedTitleView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for UnresolvedTitleProps {
    type View = UnresolvedTitleView;
}
