use dioxus::prelude::*;

/// The unresolved-abilities section heading text.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedTitleProps {
    pub text: &'static str,
}
