use super::state::GridTileState;
use dioxus::prelude::*;
use warcraft_api::Race;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, RowIndex};

/// Everything the base command tile needs to render. It is purely presentational
/// and entirely inert: it draws the look of its state, its icon or label, its race
/// accent, and its coordinate attributes. It has no hotkey, no drag state, no
/// focus, and no event handlers — `GridEditorTile` layers all of that on top by
/// wrapping this base tile. Its address is the domain `GridCoordinate`.
#[derive(Props, Clone, PartialEq)]
pub struct GridTileProps {
    /// Where the tile sits, emitted as `data-grid-row`/`-col`.
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    /// The owning unit's race, theming the accent via `data-race`.
    #[props(default = Race::Neutral)]
    pub race: Race,
    /// Ability icon URL, drawn filling the tile when present.
    #[props(default)]
    pub icon: Option<String>,
    /// Shown centered when the tile has a label and no icon.
    #[props(default)]
    pub label: String,
    #[props(default)]
    pub state: GridTileState,
}
