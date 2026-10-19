use super::model::RaceTabStateModel;
use dioxus::prelude::*;

pub(super) struct RaceTabStatePresentation {
    pub(super) label: String,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
    pub(super) accent_hidden: bool,
}

impl From<&RaceTabStateModel> for RaceTabStatePresentation {
    fn from(props: &RaceTabStateModel) -> Self {
        let label = props.label.clone();
        let onclick = props.onclick;
        let onkeydown = props.onkeydown;
        let accent_hidden = !props.is_active;
        Self {
            label,
            onclick,
            onkeydown,
            accent_hidden,
        }
    }
}

impl ddd::Layered for RaceTabStatePresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for RaceTabStatePresentation {
    type Model = RaceTabStateModel;
}
