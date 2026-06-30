use dioxus::prelude::*;

use crate::components::dialogs::dialog::DialogProps;

/// The panel's inputs: the open signal (forwarded so the header can close), the
/// title, and the body. Built from the shell props.
#[derive(Props, Clone, PartialEq)]
pub struct DialogPanelProps {
    pub open: Signal<bool>,
    pub title: String,
    pub children: Element,
    pub footer: Option<Element>,
}

impl From<&DialogProps> for DialogPanelProps {
    fn from(props: &DialogProps) -> Self {
        let open = props.open;
        let title = props.title.clone();
        let children = props.children.clone();
        let footer = props.footer.clone();
        Self {
            open,
            title,
            children,
            footer,
        }
    }
}
