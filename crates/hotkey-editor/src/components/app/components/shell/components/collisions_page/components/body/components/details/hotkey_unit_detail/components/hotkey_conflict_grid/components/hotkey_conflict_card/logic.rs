use super::props::HotkeyConflictCardProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbilityProps;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_marker_view::ConflictMarker;

impl From<&HotkeyConflictCardProps> for ConflictCardModel {
    fn from(props: &HotkeyConflictCardProps) -> Self {
        let role_label = props.conflict.role_label().to_owned();
        let hotkey_label = props.conflict.hotkey_label().to_owned();
        let abilities: Vec<ConflictAbilityProps> = props
            .conflict
            .abilities()
            .iter()
            .map(|ability| ConflictAbilityProps {
                ability_name: ability.name().to_owned(),
                ability_id: ability.object_id(),
                icon_url: ability.icon_url().map(str::to_owned),
                unit_id: props.unit_id,
                view_navigation: props.view_navigation,
            })
            .collect();
        let marker = ConflictMarker::Hotkey {
            label: hotkey_label,
        };
        Self::new(role_label, marker, abilities)
    }
}
