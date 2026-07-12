use super::view::HelpSectionTitleView;
use dioxus::prelude::*;

/// The section title's only input: the heading text.
#[derive(Props, Clone, PartialEq)]
pub struct HelpSectionTitleModel {
    #[props(into)]
    pub title: String,
}

impl From<&HelpSectionTitleView> for HelpSectionTitleModel {
    fn from(view: &HelpSectionTitleView) -> Self {
        let HelpSectionTitleView { title } = view.clone();
        Self { title }
    }
}

impl ddd::Model for HelpSectionTitleModel {
    type View = HelpSectionTitleView;
}
