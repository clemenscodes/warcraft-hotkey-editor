use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, GridLayout};

use crate::services::customkeys::upload_status::UploadStatus;

use super::super::burger_menu::BurgerMenuProps;
use super::super::collisions_button::CollisionsButtonProps;
use super::super::header_toolbar::HeaderToolbarProps;

/// The right-aligned action cluster: the collisions button (always), and either
/// the inline toolbar (full layout) or the burger drawer (compact layout).
/// Navigation and overlay open state are app-wide context, so only the editor
/// state the leaves genuinely need is threaded here.
#[derive(Props, Clone, PartialEq)]
pub struct HeaderActionsProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub grid_layout: Signal<GridLayout>,
}

impl From<&HeaderActionsProps> for CollisionsButtonProps {
    fn from(props: &HeaderActionsProps) -> Self {
        let loaded_keys = props.loaded_keys;
        let grid_layout = props.grid_layout;
        Self {
            loaded_keys,
            grid_layout,
        }
    }
}

impl From<&HeaderActionsProps> for HeaderToolbarProps {
    fn from(props: &HeaderActionsProps) -> Self {
        let loaded_keys = props.loaded_keys;
        let upload_status = props.upload_status;
        Self {
            loaded_keys,
            upload_status,
        }
    }
}

impl From<&HeaderActionsProps> for BurgerMenuProps {
    fn from(props: &HeaderActionsProps) -> Self {
        let loaded_keys = props.loaded_keys;
        Self { loaded_keys }
    }
}
