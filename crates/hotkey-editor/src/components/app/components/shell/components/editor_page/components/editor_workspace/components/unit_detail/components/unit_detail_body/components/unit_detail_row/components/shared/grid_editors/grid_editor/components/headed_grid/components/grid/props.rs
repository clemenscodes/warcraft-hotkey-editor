use super::grid_tile_kind::GridTileKind;
use dioxus::prelude::*;
use warcraft_keybinds::COMMAND_GRID_TILE_COUNT;

/// The grid's input: the finished tiles to lay out, bound to a concrete
/// [`GridTileKind`]. `Grid` encodes the command grid's shape — three rows by four
/// columns of tile squares, always exactly `COMMAND_GRID_TILE_COUNT` of them — and
/// nothing else. It is generic over the kind, so it does not know or care whether
/// the squares are interactive editor tiles or plain read-only base tiles; the
/// bound kind renders each one. The zero-sized `kind` marker carries `B` so the
/// generic is inferable, exactly as `GridEditorProps` carries its behavior.
#[derive(Props, Clone, PartialEq)]
pub struct GridProps<B: GridTileKind> {
    pub kind: B,
    pub tiles: [B::Tile; COMMAND_GRID_TILE_COUNT],
}
