use super::components::carriers_dialog_body::CarriersDialogBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The carriers dialog's bordered box: the header row above the scrolling grid of carrier
/// cards, wrapped in the library `DialogContent` (which carries no project class — this
/// panel's own classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct CarriersDialogPanelProps {
    pub header: DialogHeaderProps,
    pub body: CarriersDialogBodyProps,
}

impl From<&CarriersDialogPanelProps> for DialogHeaderProps {
    fn from(props: &CarriersDialogPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&CarriersDialogPanelProps> for CarriersDialogBodyProps {
    fn from(props: &CarriersDialogPanelProps) -> Self {
        props.body.clone()
    }
}
