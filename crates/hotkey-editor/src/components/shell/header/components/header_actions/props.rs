use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, GridLayout};

use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::navigation::view_navigation::ViewNavigationContext;

use super::super::burger_menu::BurgerMenuProps;
use super::super::collisions_button::CollisionsButtonProps;
use super::super::header_toolbar::HeaderToolbarProps;

/// The right-aligned action cluster: the collisions button (always), and either
/// the inline toolbar (full layout) or the burger drawer (compact layout).
#[derive(Props, Clone, PartialEq)]
pub struct HeaderActionsProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub preview_open: Signal<bool>,
    pub grid_layout: Signal<GridLayout>,
    pub layout_dialog_open: Signal<bool>,
    pub templates_dialog_open: Signal<bool>,
    pub system_hotkeys_open: Signal<bool>,
    pub help_open: Signal<bool>,
    pub navigation: ViewNavigationContext,
}

impl From<&HeaderActionsProps> for CollisionsButtonProps {
    fn from(props: &HeaderActionsProps) -> Self {
        let loaded_keys = props.loaded_keys;
        let grid_layout = props.grid_layout;
        let navigation = props.navigation;
        Self {
            loaded_keys,
            grid_layout,
            navigation,
        }
    }
}

impl From<&HeaderActionsProps> for HeaderToolbarProps {
    fn from(props: &HeaderActionsProps) -> Self {
        let loaded_keys = props.loaded_keys;
        let upload_status = props.upload_status;
        let preview_open = props.preview_open;
        let templates_dialog_open = props.templates_dialog_open;
        let system_hotkeys_open = props.system_hotkeys_open;
        let help_open = props.help_open;
        let navigation = props.navigation;
        Self {
            loaded_keys,
            upload_status,
            preview_open,
            templates_dialog_open,
            system_hotkeys_open,
            help_open,
            navigation,
        }
    }
}

impl From<&HeaderActionsProps> for BurgerMenuProps {
    fn from(props: &HeaderActionsProps) -> Self {
        let loaded_keys = props.loaded_keys;
        let preview_open = props.preview_open;
        let layout_dialog_open = props.layout_dialog_open;
        let templates_dialog_open = props.templates_dialog_open;
        let system_hotkeys_open = props.system_hotkeys_open;
        let help_open = props.help_open;
        let navigation = props.navigation;
        Self {
            loaded_keys,
            preview_open,
            layout_dialog_open,
            templates_dialog_open,
            system_hotkeys_open,
            help_open,
            navigation,
        }
    }
}
