use super::super::collision_card::CollisionCardProps;
use super::super::collision_count::CollisionCount;
use super::super::conflict_object_id::ConflictObjectId;
use super::super::hotkey_unit_name::HotkeyUnitName;
use super::super::hotkey_unit_row_icon::{HotkeyUnitRowIcon, HotkeyUnitRowIconProps};
use super::super::row_meta::RowMeta;
use super::props::HotkeyUnitSidebarProps;
use dioxus::prelude::*;

/// One finished card per clashing unit: its portrait, name, id, and clash count.
pub(super) fn cards(props: &HotkeyUnitSidebarProps) -> Vec<CollisionCardProps> {
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
            let unit_id_label = unit.unit_id().to_owned();
            let collision_count = unit_view.collision_count();
            let noun = if collision_count == 1 {
                "collision"
            } else {
                "collisions"
            };
            let count_text = format!("{collision_count} {noun}");
            let key_for_click = collision_key.clone();
            let onclick = EventHandler::new(move |_event: MouseEvent| {
                selected_unit.set(Some(key_for_click.clone()))
            });
            let icon = icon_url.map(|src| HotkeyUnitRowIconProps {
                src,
                alt: name.clone(),
            });
            let children = rsx! {
                if let Some(icon) = icon {
                    HotkeyUnitRowIcon { ..icon }
                }
                RowMeta {
                    HotkeyUnitName { text: name }
                    ConflictObjectId { text: unit_id_label }
                    CollisionCount { text: count_text }
                }
            };
            CollisionCardProps {
                is_selected,
                collision_key,
                onclick,
                children,
            }
        })
        .collect()
}
