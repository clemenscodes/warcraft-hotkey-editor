use super::props::HotkeyUnitSidebarProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::sidebars::shared::unit_card::UnitCardProps;
use dioxus::prelude::*;

/// One card's data per clashing unit: its portrait, name, id, and clash count.
pub(super) fn cards(props: &HotkeyUnitSidebarProps) -> Vec<UnitCardProps> {
    let mut selected_unit = props.selected_unit;
    let selected_key = selected_unit.read().clone();
    props
        .units
        .iter()
        .map(|unit_view| {
            let collision_key = unit_view.key().to_owned();
            let is_selected = selected_key.as_deref() == Some(unit_view.key());
            let unit = unit_view.unit();
            let icon_url = unit.icon_url().map(str::to_owned);
            let name = unit.name().to_owned();
            let unit_id = unit.unit_id().to_owned();
            let collision_count = unit_view.collision_count();
            let key_for_click = collision_key.clone();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                selected_unit.set(Some(key_for_click.clone()))
            });
            UnitCardProps {
                is_selected,
                collision_key,
                onclick,
                icon_url,
                name,
                unit_id,
                count: collision_count,
            }
        })
        .collect()
}
