use super::props::UnitTileOverrideProps;
use crate::components::tile_override::TileOverrideProps;

impl From<&UnitTileOverrideProps> for TileOverrideProps {
    /// Only called after the body guards that `detail` is present.
    fn from(props: &UnitTileOverrideProps) -> Self {
        let detail = props
            .detail
            .clone()
            .expect("guarded to Some before conversion");
        let loaded_keys = props.loaded_keys;
        let grid_layout = props.grid_layout;
        let selected_from_research = props.selected_from_research;
        let selected_from_uprooted = props.selected_from_uprooted;
        let tier_overrides = props.tier_overrides;
        let dragging_slot = props.dragging_slot;
        let drop_target_tile = props.drop_target_tile;
        let drag_follower = props.drag_follower;
        let active_container_slots = props.active_container_slots.clone();
        let hotkey_assign_request = props.hotkey_assign_request;
        Self {
            detail,
            loaded_keys,
            grid_layout,
            selected_from_research,
            selected_from_uprooted,
            tier_overrides,
            dragging_slot,
            drop_target_tile,
            drag_follower,
            active_container_slots,
            hotkey_assign_request,
        }
    }
}
