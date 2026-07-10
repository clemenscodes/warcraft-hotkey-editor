use super::props::HotkeyConflictCardProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::{
    ConflictAbilityData, ConflictCardModel,
};
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_marker_view::ConflictMarker;

impl From<&HotkeyConflictCardProps> for ConflictCardModel {
    fn from(props: &HotkeyConflictCardProps) -> Self {
        let role_label = props.conflict.role_label().to_owned();
        let hotkey_label = props.conflict.hotkey_label().to_owned();
        let abilities: Vec<ConflictAbilityData> = props
            .conflict
            .abilities()
            .iter()
            .map(|ability| ConflictAbilityData {
                name: ability.name().to_owned(),
                ability_id: ability.object_id(),
                icon_url: ability.icon_url().map(str::to_owned),
                unit_id: props.unit_id,
            })
            .collect();
        let marker = ConflictMarker::Hotkey {
            label: hotkey_label,
        };
        Self::new(role_label, marker, abilities)
    }
}
