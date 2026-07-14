use super::view::HelpTermView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpTermModel {
    #[props(into)]
    pub term: String,
}

impl From<&HelpTermView> for HelpTermModel {
    fn from(view: &HelpTermView) -> Self {
        let HelpTermView { term } = view.clone();
        Self { term }
    }
}

impl ddd::Model for HelpTermModel {
    type View = HelpTermView;
}
