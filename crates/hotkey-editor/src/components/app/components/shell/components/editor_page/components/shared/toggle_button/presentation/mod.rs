use super::model::ToggleButtonModel;
use dioxus::prelude::*;

pub(super) struct ToggleButtonPresentation {
    pub(super) label: &'static str,
    pub(super) title: Option<&'static str>,
    pub(super) pressed: &'static str,
    pub(super) onclick: EventHandler<MouseEvent>,
    pub(super) onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ToggleButtonModel> for ToggleButtonPresentation {
    fn from(props: &ToggleButtonModel) -> Self {
        let label = props.label;
        let title = props.title;
        let pressed = match props.active {
            true => "true",
            false => "false",
        };
        let onclick = props.onclick;
        let onkeydown = props.onkeydown;
        Self {
            label,
            title,
            pressed,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Layered for ToggleButtonPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for ToggleButtonPresentation {
    type Model = ToggleButtonModel;
}
