use super::view::UnresolvedTitleView;
use dioxus::prelude::*;

/// The unresolved-abilities section heading text.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedTitleModel {
    pub text: &'static str,
}

impl From<&UnresolvedTitleView> for UnresolvedTitleModel {
    fn from(view: &UnresolvedTitleView) -> Self {
        let UnresolvedTitleView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for UnresolvedTitleModel {
    type View = UnresolvedTitleView;
}
