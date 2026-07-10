use dioxus::prelude::*;

/// The info dialog's bordered box inputs: the header title and close handler above
/// the scroll-region body's intro, warning, and action data. Wrapped in the library
/// `DialogContent` (which carries no project class — this panel's own classed `div`
/// is the box).
#[derive(Props, Clone, PartialEq)]
pub struct InfoDialogPanelProps {
    pub title: &'static str,
    pub on_close: EventHandler<()>,
    pub intro: &'static str,
    pub warning: Option<&'static str>,
    pub primary_label: &'static str,
    pub on_primary: EventHandler<MouseEvent>,
    pub on_cancel: EventHandler<MouseEvent>,
}
