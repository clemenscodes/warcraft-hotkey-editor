use super::props::GridTileProps;
use warcraft_api::RaceLabels;

/// The presentational chrome the filled and empty base tiles render: the race
/// accent and the coordinate attributes. Field names match the attributes they
/// feed, so each tile spreads them with RSX shorthand. The base tile is inert — it
/// carries no interaction. Focus, drag, and every event handler belong to
/// `GridEditorTile`, which wraps this base tile.
#[derive(Clone, PartialEq)]
pub struct TileChrome {
    pub(super) race_attribute: &'static str,
    pub(super) row: u8,
    pub(super) column: u8,
}

impl From<&GridTileProps> for TileChrome {
    fn from(props: &GridTileProps) -> Self {
        let race_attribute = RaceLabels::data_attribute(props.race);
        let column_index = props.coordinate.column();
        let row_index = props.coordinate.row();
        let column = u8::from(column_index);
        let row = u8::from(row_index);
        Self {
            race_attribute,
            row,
            column,
        }
    }
}
