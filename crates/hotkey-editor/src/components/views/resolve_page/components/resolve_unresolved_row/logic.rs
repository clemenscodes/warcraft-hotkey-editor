use super::props::ResolveUnresolvedRowProps;
use crate::components::views::resolve_page::components::resolve_ability_icon::ResolveAbilityIconProps;
use crate::components::views::resolve_page::components::resolve_fight_name_plate::ResolveFightNamePlateProps;
use crate::components::views::resolve_page::logic::MiniGridPlacement;

/// The stuck card's plate, icon, and the single cell it is stuck on.
pub(super) struct ResolveUnresolvedRowModel {
    pub(super) name_plate: ResolveFightNamePlateProps,
    pub(super) ability: ResolveAbilityIconProps,
    pub(super) placements: Vec<MiniGridPlacement>,
}

impl From<&ResolveUnresolvedRowProps> for ResolveUnresolvedRowModel {
    fn from(props: &ResolveUnresolvedRowProps) -> Self {
        let view = props.unresolved_view.clone();
        let ability = view.ability;
        let name_plate = ResolveFightNamePlateProps {
            name: ability.name.clone(),
            object_id: ability.object_id.clone(),
        };
        let placement = MiniGridPlacement {
            column: view.column,
            row: view.row,
            icon_url: ability.icon_url.clone(),
            name: ability.name.clone(),
        };
        let placements: Vec<MiniGridPlacement> = vec![placement];
        let icon = ResolveAbilityIconProps {
            name: ability.name,
            icon_url: ability.icon_url,
            carrier_count: view.carrier_count,
            carrier_unit_ids: view.carrier_unit_ids,
            is_winner: false,
            carriers_dialog: props.carriers_dialog,
        };
        Self {
            name_plate,
            ability: icon,
            placements,
        }
    }
}
