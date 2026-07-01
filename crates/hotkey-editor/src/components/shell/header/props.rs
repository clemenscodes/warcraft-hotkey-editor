use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::{CustomKeys, GridLayout};

use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub preview_open: Signal<bool>,
    pub grid_layout: Signal<GridLayout>,
    pub system_hotkeys_open: Signal<bool>,
    pub help_open: Signal<bool>,
    pub layout_dialog_open: Signal<bool>,
    pub templates_dialog_open: Signal<bool>,
    pub current_view: Signal<AppView>,
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<String>>,
    pub search_query: Signal<String>,
}

impl From<&HeaderProps> for ViewNavigationContext {
    fn from(props: &HeaderProps) -> Self {
        let current_view = props.current_view;
        let active_race = props.active_race;
        let unit_mode = props.unit_mode;
        let selected_unit_id = props.selected_unit_id;
        let search_query = props.search_query;
        Self {
            current_view,
            active_race,
            unit_mode,
            selected_unit_id,
            search_query,
        }
    }
}
