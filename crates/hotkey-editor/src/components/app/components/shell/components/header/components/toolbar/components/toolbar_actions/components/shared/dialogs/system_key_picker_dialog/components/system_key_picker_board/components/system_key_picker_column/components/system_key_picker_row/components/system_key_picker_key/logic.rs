use super::props::SystemKeyPickerKeyProps;
use super::style;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::tooltip::{
    TooltipAnchor, TooltipPlacement, TooltipProps,
};
use dioxus::prelude::*;
use tw_macro::ClassList;

/// A system-board key's shaped presentation: the state class, the label, the wide
/// flag, and the click handler. Built by `From` so the body only places these. The
/// key's tooltip is a separate `From` into `TooltipProps`, which turns the incoming
/// placement/anchor tokens into the shared [`Tooltip`](crate) leaf's typed enums.
pub(super) struct SystemKeyPickerKeyPresentation {
    pub(super) class: ClassList,
    pub(super) label: &'static str,
    pub(super) wide: &'static str,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&SystemKeyPickerKeyProps> for SystemKeyPickerKeyPresentation {
    fn from(props: &SystemKeyPickerKeyProps) -> Self {
        let class = style::class(props.state);
        let label = props.label;
        let wide = props.wide;
        let code = props.code;
        let on_pick = props.on_pick;
        let onclick = EventHandler::new(move |_event: MouseEvent| on_pick.call(code));
        Self {
            class,
            label,
            wide,
            onclick,
        }
    }
}

impl From<&SystemKeyPickerKeyProps> for TooltipProps {
    fn from(props: &SystemKeyPickerKeyProps) -> Self {
        let text = props.title.clone();
        let placement = match props.placement {
            "above" => TooltipPlacement::Above,
            _ => TooltipPlacement::Below,
        };
        let anchor = match props.anchor {
            "left" => TooltipAnchor::Left,
            "right" => TooltipAnchor::Right,
            _ => TooltipAnchor::Center,
        };
        Self {
            text,
            placement,
            anchor,
        }
    }
}
