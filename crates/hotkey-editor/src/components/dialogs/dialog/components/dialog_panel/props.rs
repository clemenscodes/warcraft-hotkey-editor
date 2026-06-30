use dioxus::prelude::*;

use crate::components::dialogs::dialog::DialogProps;

/// The panel's inputs: the open signal (forwarded so the header can close), the
/// title, the resolved panel class, and the body. Built from the shell props.
#[derive(Props, Clone, PartialEq)]
pub struct DialogPanelProps {
    pub open: Signal<bool>,
    pub title: String,
    pub panel_class: String,
    pub children: Element,
}

impl From<&DialogProps> for DialogPanelProps {
    fn from(props: &DialogProps) -> Self {
        let open = props.open;
        let title = props.title.clone();
        let variant_class = props.panel_class.clone();
        let panel_class = format!("dialog-panel {variant_class}");
        let children = props.children.clone();
        Self {
            open,
            title,
            panel_class,
            children,
        }
    }
}
