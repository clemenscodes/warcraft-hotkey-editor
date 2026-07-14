use dioxus::prelude::*;

pub(super) struct SystemHotkeysDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
}

impl From<&SystemHotkeysDialogPresentation> for SystemHotkeysDialogShell {
    fn from(model: &SystemHotkeysDialogPresentation) -> Self {
        let open = model.open;
        let on_open_change = model.on_open_change;
        let title = String::from("System Hotkeys");
        Self {
            open,
            on_open_change,
            title,
        }
    }
}
use super::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::inventory_hotkeys_view::components::inventory_grid::InventoryDragFollower;
use super::model::SystemHotkeysDialogModel;
use super::state::SystemHotkeysDialogState;
use warcraft_api::SystemHotkeysCategory;
use warcraft_keybinds::WarcraftObjectId;

pub(super) struct SystemHotkeysDialogPresentation {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
}

impl ddd::Presentation for SystemHotkeysDialogPresentation {
    type Model = SystemHotkeysDialogModel;
}

pub(super) fn use_system_hotkeys_dialog(
    props: &SystemHotkeysDialogModel,
) -> SystemHotkeysDialogPresentation {
    let open = props.open;
    let on_open_change = props.on_open_change;
    let active_category = use_signal(|| SystemHotkeysCategory::Inventory);
    let editing_section = use_signal::<Option<WarcraftObjectId>>(|| None);
    let drag_follower = use_signal::<Option<InventoryDragFollower>>(|| None);
    let state = SystemHotkeysDialogState::new(active_category, editing_section, drag_follower);
    use_context_provider(|| state);
    SystemHotkeysDialogPresentation {
        open,
        on_open_change,
    }
}
