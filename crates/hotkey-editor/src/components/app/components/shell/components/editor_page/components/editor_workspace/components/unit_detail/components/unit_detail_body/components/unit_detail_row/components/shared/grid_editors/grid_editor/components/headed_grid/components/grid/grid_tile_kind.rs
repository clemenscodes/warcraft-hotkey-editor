use dioxus::prelude::*;

/// The kind of tile a generic [`super::Grid`] lays out in each of its twelve
/// cells. A zero-sized marker, the grid's counterpart to the domain
/// `GridBehavior`: the base grid encodes the three-by-four tile-square shape once
/// and is generic over this kind, so it stays entirely agnostic to what fills that
/// shape. Each extension binds its own kind — the editor renders an interactive
/// `GridEditorTile`, the mini grid renders a highlighted base `GridTile` — and the
/// grid just arranges whatever the kind produces.
pub trait GridTileKind: Clone + PartialEq + Default + 'static {
    /// The props of the concrete tile this kind renders.
    type Tile: Clone + PartialEq + 'static;

    /// Render one tile of this kind from its props.
    fn tile(tile: Self::Tile) -> Element;
}
