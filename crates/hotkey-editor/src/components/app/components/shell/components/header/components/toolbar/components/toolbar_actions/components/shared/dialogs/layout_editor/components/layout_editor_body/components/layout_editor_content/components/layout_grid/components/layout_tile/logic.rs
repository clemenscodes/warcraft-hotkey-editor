use super::props::LayoutTileProps;

/// The zero-based grid address the tile exposes as `data-layout-row` /
/// `data-layout-col`. The domain `GridCoordinate` arrives through the props; the
/// display `u8` is materialized here at the leaf, mirroring `grid_tile`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LayoutTileAddress {
    pub(super) row: u8,
    pub(super) column: u8,
}

impl From<&LayoutTileProps> for LayoutTileAddress {
    fn from(props: &LayoutTileProps) -> Self {
        let column_index = props.coordinate.column();
        let row_index = props.coordinate.row();
        let column = u8::from(column_index);
        let row = u8::from(row_index);
        Self { row, column }
    }
}
