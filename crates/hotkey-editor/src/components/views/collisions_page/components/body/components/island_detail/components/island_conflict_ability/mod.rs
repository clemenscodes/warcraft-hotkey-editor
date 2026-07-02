pub mod components;
mod props;
mod style;

use super::super::super::conflict_ability_icon::{ConflictAbilityIcon, ConflictAbilityIconProps};
use super::super::super::conflict_ability_name::ConflictAbilityName;
use super::super::super::conflict_ability_trigger::ConflictAbilityTrigger;
use super::super::super::conflict_object_id::ConflictObjectId;
use crate::assert_component;
use crate::components::views::collisions_page::logic::CarrierDialogData;
use components::conflict_more::ConflictMore;
use dioxus::prelude::*;
pub use props::IslandConflictAbilityProps;
use style::CLASS;
assert_component!(IslandConflictAbility);

/// One ability column of an island conflict: a clickable icon over the name and id,
/// with an optional "+N more" link. Both the icon and the link open the same
/// carriers dialog for this ability.
#[component]
pub fn IslandConflictAbility(props: IslandConflictAbilityProps) -> Element {
    let ability_name = props.ability_name;
    let ability_id = props.ability_id;
    let extra_count = props.extra_count;
    let mut carrier_dialog = props.carrier_dialog;
    let icon = ConflictAbilityIconProps {
        src: props.icon_url,
        alt: ability_name.clone(),
    };
    let open_dialog_name = ability_name.clone();
    let carrier_unit_ids = props.carrier_unit_ids;
    let open = move |_event: MouseEvent| {
        let data = CarrierDialogData::new(open_dialog_name.clone(), &carrier_unit_ids);
        carrier_dialog.set(Some(data));
    };
    let open_from_icon = EventHandler::new(open.clone());
    let open_from_more = EventHandler::new(open);
    rsx! {
        div {
            class: CLASS,
            ConflictAbilityTrigger {
                onclick: open_from_icon,
                ConflictAbilityIcon { ..icon }
            }
            ConflictAbilityName { text: ability_name }
            ConflictObjectId { text: ability_id }
            if extra_count > 0 {
                ConflictMore { count: extra_count, onclick: open_from_more }
            }
        }
    }
}
