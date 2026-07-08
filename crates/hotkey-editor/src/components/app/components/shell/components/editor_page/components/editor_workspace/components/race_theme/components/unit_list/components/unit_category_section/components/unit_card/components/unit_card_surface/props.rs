use super::components::unit_card_icon::UnitCardIconProps;
use super::components::unit_card_info::UnitCardInfoProps;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::Race;

/// The unit card's selectable button surface: the portrait plus name/id it lays out,
/// the selected flag that drives its `--race-color` accent, and the click, keydown,
/// and mount handlers the card wires straight onto its own button (click and
/// Space/Enter select the unit; the mount registers the button with the focus
/// coordinator).
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardSurfaceProps {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    #[props(into)]
    pub unit_id: String,
    pub race: Race,
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
    pub onmounted: EventHandler<Event<MountedData>>,
}

impl From<&UnitCardSurfaceProps> for UnitCardIconProps {
    fn from(props: &UnitCardSurfaceProps) -> Self {
        let icon_path = props.icon_path.clone();
        let display_name = props.display_name.clone();
        Self {
            icon_path,
            display_name,
        }
    }
}

impl From<&UnitCardSurfaceProps> for UnitCardInfoProps {
    fn from(props: &UnitCardSurfaceProps) -> Self {
        let display_name = props.display_name.clone();
        let unit_id = props.unit_id.clone();
        let race = props.race;
        let is_selected = props.is_selected;
        Self {
            display_name,
            unit_id,
            race,
            is_selected,
        }
    }
}
