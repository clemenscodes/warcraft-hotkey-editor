use dioxus::prelude::*;

/// The intro line's copy.
#[derive(Props, Clone, PartialEq)]
pub struct InfoIntroProps {
    pub intro: &'static str,
}
