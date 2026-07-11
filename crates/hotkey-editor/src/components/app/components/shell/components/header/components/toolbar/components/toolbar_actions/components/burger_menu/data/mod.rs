use super::state::BurgerAction;
use crate::components::app::components::shell::components::shared::icons::{
    ICON_COG, ICON_DOWNLOAD, ICON_GRID, ICON_HELP, ICON_PREVIEW, ICON_REDO, ICON_RESOLVE,
    ICON_TEMPLATES, ICON_UNDO, ICON_UPLOAD,
};

/// The static, content-only attributes of one drawer row: everything that does
/// not depend on live signal state. The dynamic pieces — disabled, active
/// weight, live label, `aria-*` flags, and the click handler — are layered on
/// per row by the builder in `logic.rs`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) struct BurgerRowContent {
    pub(super) action: BurgerAction,
    pub(super) icon: &'static str,
    pub(super) label: &'static str,
    pub(super) role: Option<&'static str>,
    pub(super) aria_haspopup: Option<&'static str>,
    pub(super) aria_label: Option<&'static str>,
}

/// The primary Grid Layout row, rendered above the grouped file-action list.
pub(super) const LAYOUT_ROW: BurgerRowContent = BurgerRowContent {
    action: BurgerAction::Layout,
    icon: ICON_GRID,
    label: "Grid Layout",
    role: None,
    aria_haspopup: Some("dialog"),
    aria_label: Some("Edit global hotkey layout"),
};

/// The grouped file-action rows, in render order.
pub(super) const ITEM_ROWS: &[BurgerRowContent] = &[
    BurgerRowContent {
        action: BurgerAction::Undo,
        icon: ICON_UNDO,
        label: "Undo",
        role: Some("menuitem"),
        aria_haspopup: None,
        aria_label: None,
    },
    BurgerRowContent {
        action: BurgerAction::Redo,
        icon: ICON_REDO,
        label: "Redo",
        role: Some("menuitem"),
        aria_haspopup: None,
        aria_label: None,
    },
    BurgerRowContent {
        action: BurgerAction::Upload,
        icon: ICON_UPLOAD,
        label: "Upload",
        role: Some("menuitem"),
        aria_haspopup: None,
        aria_label: None,
    },
    BurgerRowContent {
        action: BurgerAction::Templates,
        icon: ICON_TEMPLATES,
        label: "Browse Templates",
        role: Some("menuitem"),
        aria_haspopup: Some("dialog"),
        aria_label: None,
    },
    BurgerRowContent {
        action: BurgerAction::SystemHotkeys,
        icon: ICON_COG,
        label: "System Hotkeys",
        role: Some("menuitem"),
        aria_haspopup: Some("dialog"),
        aria_label: None,
    },
    BurgerRowContent {
        action: BurgerAction::Preview,
        icon: ICON_PREVIEW,
        label: "Preview",
        role: Some("menuitem"),
        aria_haspopup: None,
        aria_label: None,
    },
    BurgerRowContent {
        action: BurgerAction::Resolve,
        icon: ICON_RESOLVE,
        label: "Resolve Conflicts",
        role: Some("menuitem"),
        aria_haspopup: None,
        aria_label: None,
    },
    BurgerRowContent {
        action: BurgerAction::Download,
        icon: ICON_DOWNLOAD,
        label: "Download",
        role: Some("menuitem"),
        aria_haspopup: None,
        aria_label: None,
    },
    BurgerRowContent {
        action: BurgerAction::Help,
        icon: ICON_HELP,
        label: "Help",
        role: Some("menuitem"),
        aria_haspopup: Some("dialog"),
        aria_label: None,
    },
];
