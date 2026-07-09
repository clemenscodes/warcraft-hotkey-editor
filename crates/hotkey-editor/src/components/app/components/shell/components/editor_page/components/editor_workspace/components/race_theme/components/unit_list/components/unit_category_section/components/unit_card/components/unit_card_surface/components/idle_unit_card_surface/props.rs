use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_icon::UnitCardIconProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_info::UnitCardInfoProps;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::{Race, WarcraftObjectId};

/// The idle unit card surface's props: the portrait and text it lays out plus the
/// card's handlers. Built by the dispatcher from `UnitCardSurfaceProps`.
#[derive(Props, Clone, PartialEq)]
pub struct IdleUnitCardSurfaceProps {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub race: Race,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onmounted: EventHandler<Event<MountedData>>,
}

impl From<&IdleUnitCardSurfaceProps> for UnitCardIconProps {
    fn from(props: &IdleUnitCardSurfaceProps) -> Self {
        let icon_path = props.icon_path.clone();
        let display_name = props.display_name.clone();
        Self {
            icon_path,
            display_name,
        }
    }
}

impl From<&IdleUnitCardSurfaceProps> for UnitCardInfoProps {
    fn from(props: &IdleUnitCardSurfaceProps) -> Self {
        let display_name = props.display_name.clone();
        let unit_id = props.unit_id;
        let race = props.race;
        let is_selected = false;
        Self {
            display_name,
            unit_id,
            race,
            is_selected,
        }
    }
}
