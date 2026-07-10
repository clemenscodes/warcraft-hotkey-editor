use dioxus::prelude::*;

/// The system-hotkeys dialog's bordered box: the header row above the scrolling body,
/// wrapped in the library `DialogContent` (which carries no project class — this panel's
/// own classed `div` is the box). The body reads its category tab, editing section, and
/// inventory drag follower from context, so the panel carries only the header's title
/// and close handler.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysDialogPanelProps {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
}
