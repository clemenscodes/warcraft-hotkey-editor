use super::view::HelpSectionTitleView;
use dioxus::prelude::*;

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
