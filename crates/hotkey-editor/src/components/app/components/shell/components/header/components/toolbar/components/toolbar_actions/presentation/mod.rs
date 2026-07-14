use crate::components::app::components::shell::components::shared::icons::{
    ICON_COG, ICON_DOWNLOAD, ICON_HELP, ICON_PREVIEW, ICON_REDO, ICON_RESOLVE, ICON_TEMPLATES,
    ICON_UNDO, ICON_UPLOAD,
};
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use crate::services::overlay_state::context::use_overlay_state;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

/// Which file action a control triggers. The single source of truth both the inline
/// toolbar buttons and the burger drawer rows resolve against.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ToolbarActionKind {
    Undo,
    Redo,
    Upload,
    Templates,
    SystemHotkeys,
    Resolve,
    Preview,
    Download,
    Help,
}

/// One file action, fully resolved once: its icon, both labels (the toolbar's accessible
/// label and the drawer's shorter visible label), its live aria/disabled/active state, and
/// its base click handler. The toolbar renders it as a square button and the burger as a
/// drawer row, each reading these fields — so an action's icon, label, and behaviour are
/// defined in exactly one place.
#[derive(Clone, PartialEq)]
pub struct ToolbarActionModel {
    pub kind: ToolbarActionKind,
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub label: String,
    pub aria_haspopup: Option<&'static str>,
    pub expanded: Option<bool>,
    pub pressed: Option<bool>,
    pub active: bool,
    pub disabled: bool,
    pub hidden: bool,
    pub onclick: EventHandler<MouseEvent>,
}

/// The nine file actions, resolved once from live state. Both looks read the same set.
#[derive(Clone, PartialEq)]
pub struct ToolbarActionSet {
    actions: Vec<ToolbarActionModel>,
}

impl ToolbarActionSet {
    /// One action by kind, for a toolbar button that renders a single fixed action.
    pub fn get(&self, kind: ToolbarActionKind) -> ToolbarActionModel {
        let found = self.actions.iter().find(|action| action.kind == kind);
        let action = found.expect("every toolbar action kind is built");
        action.clone()
    }

    /// Every action in render order, for the burger drawer's row list.
    pub fn items(&self) -> Vec<ToolbarActionModel> {
        self.actions.clone()
    }
}

/// Reads the undo history, navigation, overlay dialogs, and document, and resolves all
/// nine file actions with their base click handlers (no drawer-close — the burger wraps
/// that on top). Each toolbar button and the burger drawer call this so an action's icon,
/// label, and behaviour live in one place; because it reads live signals, every consumer
/// re-resolves reactively.
pub(crate) fn use_toolbar_actions() -> ToolbarActionSet {
    let navigation = use_view_navigation();
    let overlay = use_overlay_state();
    let history = use_undo_history();
    let custom_keys_service = use_custom_keys_service();

    let keys = custom_keys_service.keys();
    let has_file_memo = use_memo(move || keys.read().is_some());
    let has_file = has_file_memo();
    let can_undo = history.can_undo();
    let can_redo = history.can_redo();
    let preview_active = *overlay.preview_open().read();
    let system_active = *overlay.system_hotkeys_open().read();
    let templates_expanded = *overlay.templates_dialog_open().read();
    let help_expanded = *overlay.help_open().read();

    let undo_onclick = EventHandler::new(move |_event: MouseEvent| history.undo());
    let redo_onclick = EventHandler::new(move |_event: MouseEvent| history.redo());
    let mut upload_open = overlay.upload_info_open();
    let upload_onclick = EventHandler::new(move |_event: MouseEvent| upload_open.set(true));
    let mut templates_open = overlay.templates_dialog_open();
    let templates_onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*templates_open.read();
        templates_open.set(next);
    });
    let mut system_open = overlay.system_hotkeys_open();
    let system_onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*system_open.read();
        system_open.set(next);
    });
    let resolve_onclick =
        EventHandler::new(move |_event: MouseEvent| navigation.apply(AppView::Resolve));
    let mut preview_open = overlay.preview_open();
    let preview_onclick = EventHandler::new(move |_event: MouseEvent| {
        let next = !*preview_open.read();
        preview_open.set(next);
    });
    let mut download_open = overlay.download_info_open();
    let download_onclick = EventHandler::new(move |_event: MouseEvent| download_open.set(true));
    let mut help_open = overlay.help_open();
    let help_onclick = EventHandler::new(move |_event: MouseEvent| help_open.set(true));

    let preview_aria_label = if preview_active {
        "Hide preview"
    } else {
        "Preview"
    };
    let preview_label = if preview_active {
        String::from("Hide Preview")
    } else {
        String::from("Preview")
    };

    let undo = ToolbarActionModel {
        kind: ToolbarActionKind::Undo,
        icon: ICON_UNDO,
        aria_label: "Undo",
        label: String::from("Undo"),
        aria_haspopup: None,
        expanded: None,
        pressed: None,
        active: false,
        disabled: !can_undo,
        hidden: false,
        onclick: undo_onclick,
    };
    let redo = ToolbarActionModel {
        kind: ToolbarActionKind::Redo,
        icon: ICON_REDO,
        aria_label: "Redo",
        label: String::from("Redo"),
        aria_haspopup: None,
        expanded: None,
        pressed: None,
        active: false,
        disabled: !can_redo,
        hidden: false,
        onclick: redo_onclick,
    };
    let upload = ToolbarActionModel {
        kind: ToolbarActionKind::Upload,
        icon: ICON_UPLOAD,
        aria_label: "Upload CustomKeys.txt",
        label: String::from("Upload"),
        aria_haspopup: None,
        expanded: None,
        pressed: None,
        active: false,
        disabled: false,
        hidden: false,
        onclick: upload_onclick,
    };
    let templates = ToolbarActionModel {
        kind: ToolbarActionKind::Templates,
        icon: ICON_TEMPLATES,
        aria_label: "Browse layout templates",
        label: String::from("Browse Templates"),
        aria_haspopup: Some("dialog"),
        expanded: Some(templates_expanded),
        pressed: None,
        active: false,
        disabled: false,
        hidden: false,
        onclick: templates_onclick,
    };
    let system_hotkeys = ToolbarActionModel {
        kind: ToolbarActionKind::SystemHotkeys,
        icon: ICON_COG,
        aria_label: "General hotkeys",
        label: String::from("System Hotkeys"),
        aria_haspopup: Some("dialog"),
        expanded: Some(system_active),
        pressed: None,
        active: system_active,
        disabled: false,
        hidden: false,
        onclick: system_onclick,
    };
    let resolve = ToolbarActionModel {
        kind: ToolbarActionKind::Resolve,
        icon: ICON_RESOLVE,
        aria_label: "Resolve conflicts",
        label: String::from("Resolve Conflicts"),
        aria_haspopup: None,
        expanded: None,
        pressed: None,
        active: false,
        disabled: !has_file,
        hidden: false,
        onclick: resolve_onclick,
    };
    let preview = ToolbarActionModel {
        kind: ToolbarActionKind::Preview,
        icon: ICON_PREVIEW,
        aria_label: preview_aria_label,
        label: preview_label,
        aria_haspopup: None,
        expanded: None,
        pressed: Some(preview_active),
        active: preview_active,
        disabled: false,
        hidden: false,
        onclick: preview_onclick,
    };
    let download = ToolbarActionModel {
        kind: ToolbarActionKind::Download,
        icon: ICON_DOWNLOAD,
        aria_label: "Download CustomKeys.txt",
        label: String::from("Download"),
        aria_haspopup: None,
        expanded: None,
        pressed: None,
        active: false,
        disabled: false,
        hidden: !has_file,
        onclick: download_onclick,
    };
    let help = ToolbarActionModel {
        kind: ToolbarActionKind::Help,
        icon: ICON_HELP,
        aria_label: "How to use this editor",
        label: String::from("Help"),
        aria_haspopup: Some("dialog"),
        expanded: Some(help_expanded),
        pressed: None,
        active: false,
        disabled: false,
        hidden: false,
        onclick: help_onclick,
    };
    let actions = vec![
        undo,
        redo,
        upload,
        templates,
        system_hotkeys,
        resolve,
        preview,
        download,
        help,
    ];
    ToolbarActionSet { actions }
}
