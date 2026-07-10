use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_icon::UnitCardIconProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_info::UnitCardInfoProps;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The selected unit card surface's props: the portrait and text it lays out plus the
/// card's handlers. Built by the dispatcher from `UnitCardSurfaceProps`.
#[derive(Props, Clone, PartialEq)]
pub struct SelectedUnitCardSurfaceProps {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&SelectedUnitCardSurfaceProps> for UnitCardIconProps {
    fn from(props: &SelectedUnitCardSurfaceProps) -> Self {
        let icon_path = props.icon_path.clone();
        let display_name = props.display_name.clone();
        Self {
            icon_path,
            display_name,
        }
    }
}

impl From<&SelectedUnitCardSurfaceProps> for UnitCardInfoProps {
    fn from(props: &SelectedUnitCardSurfaceProps) -> Self {
        let display_name = props.display_name.clone();
        let unit_id = props.unit_id;
        let is_selected = true;
        Self {
            display_name,
            unit_id,
            is_selected,
        }
    }
}
