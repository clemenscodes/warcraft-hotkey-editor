use dioxus::prelude::*;

use crate::components::shared::icons::{
    ICON_COG, ICON_DOWNLOAD, ICON_GRID, ICON_HELP, ICON_PREVIEW, ICON_REDO, ICON_RESOLVE,
    ICON_TEMPLATES, ICON_UNDO, ICON_UPLOAD,
};
use crate::services::files::download::BlobDownload;
use crate::services::navigation::app_view::AppView;
use crate::services::undo::UndoHistory;

use super::components::burger_drawer::BurgerDrawerProps;
use super::components::burger_menu_item::{BurgerItemState, BurgerMenuItemProps};
use super::props::BurgerMenuProps;

/// The already-shaped controller state the body renders: the drawer open flags,
/// the toggle and download handlers, and the fully-built drawer props (primary
/// row plus grouped action rows, each with its handler and state).
pub struct BurgerMenuView {
    pub burger_open: Signal<bool>,
    pub upload_info_open: Signal<bool>,
    pub download_info_open: Signal<bool>,
    pub has_loaded_file: bool,
    pub toggle: EventHandler<MouseEvent>,
    pub download_confirm: EventHandler<()>,
    pub drawer: BurgerDrawerProps,
}

/// The composed hook: owns the drawer's local open state, reads the undo history
/// and the live config, and wires every row's handler and state. The body only
/// names the result.
pub fn use_burger_menu(props: &BurgerMenuProps) -> BurgerMenuView {
    let loaded_keys = props.loaded_keys;
    let navigation = props.navigation;
    let mut system_hotkeys_open = props.system_hotkeys_open;
    let mut help_open = props.help_open;
    let mut layout_dialog_open = props.layout_dialog_open;
    let mut templates_dialog_open = props.templates_dialog_open;
    let mut preview_open = props.preview_open;
    let mut burger_open = use_signal::<bool>(|| false);
    let mut upload_info_open = use_signal::<bool>(|| false);
    let mut download_info_open = use_signal::<bool>(|| false);

    let has_loaded_file = loaded_keys.read().is_some();
    let preview_active = preview_open();
    let system_hotkeys_active = system_hotkeys_open();

    let history = use_context::<UndoHistory>();
    let can_undo = history.can_undo();
    let can_redo = history.can_redo();

    let toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*burger_open.read();
        burger_open.set(next);
    });
    let close = EventHandler::new(move |_event: MouseEvent| burger_open.set(false));

    let toggle_layout = EventHandler::new(move |_event: MouseEvent| {
        let next = !*layout_dialog_open.read();
        layout_dialog_open.set(next);
        burger_open.set(false);
    });
    let open_upload = EventHandler::new(move |_event: MouseEvent| {
        upload_info_open.set(true);
        burger_open.set(false);
    });
    let toggle_templates = EventHandler::new(move |_event: MouseEvent| {
        let next = !*templates_dialog_open.read();
        templates_dialog_open.set(next);
        burger_open.set(false);
    });
    let toggle_system_hotkeys = EventHandler::new(move |_event: MouseEvent| {
        let next = !*system_hotkeys_open.read();
        system_hotkeys_open.set(next);
        burger_open.set(false);
    });
    let open_help = EventHandler::new(move |_event: MouseEvent| {
        help_open.set(true);
        burger_open.set(false);
    });
    let toggle_preview = EventHandler::new(move |_event: MouseEvent| {
        let next = !*preview_open.read();
        preview_open.set(next);
        burger_open.set(false);
    });
    let open_download = EventHandler::new(move |_event: MouseEvent| {
        download_info_open.set(true);
        burger_open.set(false);
    });
    let open_resolve = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        navigation.apply(AppView::Resolve);
    });
    let trigger_undo = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        history.undo();
    });
    let trigger_redo = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        history.redo();
    });
    let download_confirm = EventHandler::new(move |_event: ()| {
        let serialized = {
            let read_guard = loaded_keys.read();
            let Some(file) = read_guard.as_ref() else {
                return;
            };
            file.normalize().to_string()
        };
        BlobDownload::trigger("CustomKeys.txt", &serialized);
    });

    let layout = BurgerMenuItemProps {
        icon: ICON_GRID,
        label: String::from("Grid Layout"),
        state: BurgerItemState::Primary,
        disabled: false,
        role: None,
        data_action: None,
        aria_haspopup: Some("dialog"),
        aria_expanded: aria_flag(layout_dialog_open()),
        aria_pressed: None,
        aria_label: Some("Edit global hotkey layout"),
        onclick: toggle_layout,
    };

    let mut items: Vec<BurgerMenuItemProps> = Vec::new();
    let undo_item = BurgerMenuItemProps {
        icon: ICON_UNDO,
        label: String::from("Undo"),
        state: BurgerItemState::Idle,
        disabled: !can_undo,
        role: Some("menuitem"),
        data_action: Some("undo"),
        aria_haspopup: None,
        aria_expanded: None,
        aria_pressed: None,
        aria_label: None,
        onclick: trigger_undo,
    };
    items.push(undo_item);
    let redo_item = BurgerMenuItemProps {
        icon: ICON_REDO,
        label: String::from("Redo"),
        state: BurgerItemState::Idle,
        disabled: !can_redo,
        role: Some("menuitem"),
        data_action: Some("redo"),
        aria_haspopup: None,
        aria_expanded: None,
        aria_pressed: None,
        aria_label: None,
        onclick: trigger_redo,
    };
    items.push(redo_item);
    let upload_item = BurgerMenuItemProps {
        icon: ICON_UPLOAD,
        label: String::from("Upload"),
        state: BurgerItemState::Idle,
        disabled: false,
        role: Some("menuitem"),
        data_action: None,
        aria_haspopup: None,
        aria_expanded: None,
        aria_pressed: None,
        aria_label: None,
        onclick: open_upload,
    };
    items.push(upload_item);
    let templates_item = BurgerMenuItemProps {
        icon: ICON_TEMPLATES,
        label: String::from("Browse Templates"),
        state: BurgerItemState::Idle,
        disabled: false,
        role: Some("menuitem"),
        data_action: None,
        aria_haspopup: Some("dialog"),
        aria_expanded: aria_flag(templates_dialog_open()),
        aria_pressed: None,
        aria_label: None,
        onclick: toggle_templates,
    };
    items.push(templates_item);
    let system_hotkeys_item = BurgerMenuItemProps {
        icon: ICON_COG,
        label: String::from("System Hotkeys"),
        state: item_state(system_hotkeys_active),
        disabled: false,
        role: Some("menuitem"),
        data_action: None,
        aria_haspopup: Some("dialog"),
        aria_expanded: aria_flag(system_hotkeys_active),
        aria_pressed: None,
        aria_label: None,
        onclick: toggle_system_hotkeys,
    };
    items.push(system_hotkeys_item);
    let preview_label = if preview_active {
        String::from("Hide Preview")
    } else {
        String::from("Preview")
    };
    let preview_item = BurgerMenuItemProps {
        icon: ICON_PREVIEW,
        label: preview_label,
        state: item_state(preview_active),
        disabled: false,
        role: Some("menuitem"),
        data_action: None,
        aria_haspopup: None,
        aria_expanded: None,
        aria_pressed: aria_flag(preview_active),
        aria_label: None,
        onclick: toggle_preview,
    };
    items.push(preview_item);
    if has_loaded_file {
        let resolve_item = BurgerMenuItemProps {
            icon: ICON_RESOLVE,
            label: String::from("Resolve Conflicts"),
            state: BurgerItemState::Idle,
            disabled: false,
            role: Some("menuitem"),
            data_action: Some("view-resolve"),
            aria_haspopup: None,
            aria_expanded: None,
            aria_pressed: None,
            aria_label: None,
            onclick: open_resolve,
        };
        items.push(resolve_item);
        let download_item = BurgerMenuItemProps {
            icon: ICON_DOWNLOAD,
            label: String::from("Download"),
            state: BurgerItemState::Idle,
            disabled: false,
            role: Some("menuitem"),
            data_action: None,
            aria_haspopup: None,
            aria_expanded: None,
            aria_pressed: None,
            aria_label: None,
            onclick: open_download,
        };
        items.push(download_item);
    }
    let help_item = BurgerMenuItemProps {
        icon: ICON_HELP,
        label: String::from("Help"),
        state: BurgerItemState::Idle,
        disabled: false,
        role: Some("menuitem"),
        data_action: None,
        aria_haspopup: Some("dialog"),
        aria_expanded: aria_flag(help_open()),
        aria_pressed: None,
        aria_label: None,
        onclick: open_help,
    };
    items.push(help_item);

    let drawer = BurgerDrawerProps {
        on_close: close,
        layout,
        items,
    };

    BurgerMenuView {
        burger_open,
        upload_info_open,
        download_info_open,
        has_loaded_file,
        toggle,
        download_confirm,
        drawer,
    }
}

/// A live boolean rendered as an `aria-*` attribute value, or omitted when the
/// row does not carry that attribute.
fn aria_flag(value: bool) -> Option<&'static str> {
    if value { Some("true") } else { Some("false") }
}

/// A toggle row is styled active while its target (dialog/preview) is open.
fn item_state(active: bool) -> BurgerItemState {
    if active {
        BurgerItemState::Active
    } else {
        BurgerItemState::Idle
    }
}
