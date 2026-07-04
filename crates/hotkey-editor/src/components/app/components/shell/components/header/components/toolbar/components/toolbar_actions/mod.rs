pub mod components;
mod style;

use crate::assert_component;
use components::burger_menu::BurgerMenu;
use components::help_dialog_host::HelpDialogHost;
use components::inline_actions::InlineActions;
use components::layout_editor_host::LayoutEditorHost;
use components::preview_dialog_host::PreviewDialogHost;
use components::system_hotkeys_dialog_host::SystemHotkeysDialogHost;
use components::templates_dialog_host::TemplatesDialogHost;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ToolbarActions);

/// The adaptive file-action controls: the inline button row at laptop width and up,
/// collapsing into the burger drawer on narrower screens, plus the overlay dialogs
/// those actions open. A layout-neutral grouping wrapper — it owns no box and threads
/// no data; each child self-sources. The dialog hosts live here, not in the shell,
/// because this is the always-mounted home of the actions that open them: the inline
/// buttons and the burger both flip the shared open signals, and a host renders the
/// matching dialog regardless of which control is visible at the current width.
#[component]
pub fn ToolbarActions() -> Element {
    rsx! {
        div {
            class: CLASS,
            InlineActions {}
            BurgerMenu {}
            PreviewDialogHost {}
            SystemHotkeysDialogHost {}
            HelpDialogHost {}
            TemplatesDialogHost {}
            LayoutEditorHost {}
        }
    }
}
