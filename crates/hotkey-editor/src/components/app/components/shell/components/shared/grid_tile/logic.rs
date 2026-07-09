use super::props::GridTileProps;

/// The presentational chrome the filled and empty base tiles render: the coordinate
/// attributes. Field names match the attributes they feed, so each tile spreads them
/// with RSX shorthand. The base tile is inert — it carries no interaction. Its race
/// accent is read from the inherited `--race-accent`, so no race is carried here. Focus,
/// drag, and every event handler belong to `GridEditorTile`, which wraps this base tile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TileChrome {
    pub(super) row: u8,
    pub(super) column: u8,
}

impl From<&GridTileProps> for TileChrome {
    fn from(props: &GridTileProps) -> Self {
        let column_index = props.coordinate.column();
        let row_index = props.coordinate.row();
        let column = u8::from(column_index);
        let row = u8::from(row_index);
        Self { row, column }
    }
}
