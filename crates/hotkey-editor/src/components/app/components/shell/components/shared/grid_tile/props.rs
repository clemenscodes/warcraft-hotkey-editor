use super::state::GridTileState;
use super::view::GridTileView;
use dioxus::prelude::*;
use warcraft_keybinds::{ColumnIndex, GridCoordinate, RowIndex};

/// The base tile's private internal model, mirroring the published [`GridTileView`].
/// Parents set these fields by name from a view they hold.
#[derive(Props, Clone, PartialEq)]
pub struct GridTileProps {
    #[props(default = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero))]
    pub coordinate: GridCoordinate,
    #[props(default)]
    pub icon: Option<String>,
    #[props(default)]
    pub label: String,
    #[props(default)]
    pub state: GridTileState,
    #[props(default)]
    pub is_dragging_source: bool,
    #[props(default)]
    pub is_drag_over: bool,
}

impl From<&GridTileView> for GridTileProps {
    fn from(view: &GridTileView) -> Self {
        let coordinate = view.coordinate;
        let icon = view.icon.clone();
        let label = view.label.clone();
        let state = view.state;
        let is_dragging_source = view.is_dragging_source;
        let is_drag_over = view.is_drag_over;
        Self {
            coordinate,
            icon,
            label,
            state,
            is_dragging_source,
            is_drag_over,
        }
    }
}

impl ddd::Props for GridTileProps {
    type View = GridTileView;
}
