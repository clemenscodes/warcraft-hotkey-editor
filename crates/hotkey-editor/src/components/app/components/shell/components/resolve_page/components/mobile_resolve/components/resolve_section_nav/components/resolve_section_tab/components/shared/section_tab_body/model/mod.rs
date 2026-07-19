use super::view::SectionTabBodyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionTabBodyModel {
    #[props(into)]
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SectionTabBodyView> for SectionTabBodyModel {
    fn from(view: &SectionTabBodyView) -> Self {
        let SectionTabBodyView {
            label,
            count,
            active,
            onclick,
        } = view.clone();
        Self {
            label,
            count,
            active,
            onclick,
        }
    }
}

impl ddd::Model for SectionTabBodyModel {
    type View = SectionTabBodyView;
}
