use super::props::UnitTileOverrideProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::TileOverrideProps;

impl From<&UnitTileOverrideProps> for TileOverrideProps {
    /// Only called after the body guards that `detail` is present.
    fn from(props: &UnitTileOverrideProps) -> Self {
        let detail = props
            .detail
            .clone()
            .expect("guarded to Some before conversion");
        let active_container_slots = props.active_container_slots.clone();
        Self {
            detail,
            active_container_slots,
        }
    }
}
