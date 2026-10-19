use super::model::SectionTabBodyModel;
use dioxus::prelude::*;

pub(super) struct SectionTabBodyPresentation {
    pub(super) text: String,
    pub(super) count: usize,
    pub(super) current: &'static str,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&SectionTabBodyModel> for SectionTabBodyPresentation {
    fn from(props: &SectionTabBodyModel) -> Self {
        let text = props.label.clone();
        let count = props.count;
        let current = match props.active {
            true => "true",
            false => "false",
        };
        let onclick = props.onclick;
        Self {
            text,
            count,
            current,
            onclick,
        }
    }
}

impl ddd::Layered for SectionTabBodyPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for SectionTabBodyPresentation {
    type Model = SectionTabBodyModel;
}
