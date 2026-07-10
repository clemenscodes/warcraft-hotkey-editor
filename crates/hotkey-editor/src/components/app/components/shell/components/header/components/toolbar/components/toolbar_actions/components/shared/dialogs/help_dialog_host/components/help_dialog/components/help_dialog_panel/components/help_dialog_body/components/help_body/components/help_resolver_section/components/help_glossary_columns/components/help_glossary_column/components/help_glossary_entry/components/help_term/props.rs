use super::view::HelpTermView;
use dioxus::prelude::*;

/// The glossary term's only input: the term text.
#[derive(Props, Clone, PartialEq)]
pub struct HelpTermProps {
    #[props(into)]
    pub term: String,
}

impl From<&HelpTermView> for HelpTermProps {
    fn from(view: &HelpTermView) -> Self {
        let HelpTermView { term } = view.clone();
        Self { term }
    }
}

impl ddd::Props for HelpTermProps {
    type View = HelpTermView;
}
