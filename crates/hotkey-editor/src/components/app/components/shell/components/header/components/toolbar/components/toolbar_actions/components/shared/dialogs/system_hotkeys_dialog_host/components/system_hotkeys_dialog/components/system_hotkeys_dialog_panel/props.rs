use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The system-hotkeys dialog's bordered box: the header row above the scrolling body,
/// wrapped in the library `DialogContent` (which carries no project class — this panel's
/// own classed `div` is the box). The body reads its category tab, editing section, and
/// inventory drag follower from context, so the panel carries only the header.
#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysDialogPanelProps {
    pub header: DialogHeaderProps,
}

impl From<&SystemHotkeysDialogPanelProps> for DialogHeaderProps {
    fn from(props: &SystemHotkeysDialogPanelProps) -> Self {
        props.header.clone()
    }
}
