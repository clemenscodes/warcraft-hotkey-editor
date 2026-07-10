use super::view::HelpSectionTitleView;
use dioxus::prelude::*;

/// The section title's only input: the heading text.
#[derive(Props, Clone, PartialEq)]
pub struct HelpSectionTitleProps {
    #[props(into)]
    pub title: String,
}

impl From<&HelpSectionTitleView> for HelpSectionTitleProps {
    fn from(view: &HelpSectionTitleView) -> Self {
        let HelpSectionTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Props for HelpSectionTitleProps {
    type View = HelpSectionTitleView;
}
