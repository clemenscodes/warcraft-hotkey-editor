use super::props::SystemKeyPickerKeyProps;
use super::style;
use dioxus::prelude::*;
use tw_macro::ClassList;

/// A system-board key's shaped presentation: the state class, the label, the
/// tooltip text and its placement/anchor, the wide flag, and the click handler.
/// Built by `From` so the body only places these.
pub(super) struct SystemKeyPickerKeyPresentation {
    pub(super) class: ClassList,
    pub(super) label: &'static str,
    pub(super) title: String,
    pub(super) placement: &'static str,
    pub(super) anchor: &'static str,
    pub(super) wide: &'static str,
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&SystemKeyPickerKeyProps> for SystemKeyPickerKeyPresentation {
    fn from(props: &SystemKeyPickerKeyProps) -> Self {
        let class = style::class(props.state);
        let label = props.label;
        let title = props.title.clone();
        let placement = props.placement;
        let anchor = props.anchor;
        let wide = props.wide;
        let code = props.code;
        let on_pick = props.on_pick;
        let onclick = EventHandler::new(move |_event: MouseEvent| on_pick.call(code));
        Self {
            class,
            label,
            title,
            placement,
            anchor,
            wide,
            onclick,
        }
    }
}
