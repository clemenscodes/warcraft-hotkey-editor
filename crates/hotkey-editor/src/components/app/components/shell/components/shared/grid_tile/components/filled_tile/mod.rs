pub mod components;
mod logic;
mod props;
mod view;

pub use view::FilledTileView;
mod style;

use super::shared::drag_over_ring::DragOverRing;
use components::ability_fill::AbilityFill;
use components::command_fill::CommandFill;
use components::dragging_source_ghost::DraggingSourceGhost;
use components::selection_ring::SelectionRing;
use components::tile_icon::TileIcon;
use components::tile_label::TileLabel;
use dioxus::prelude::*;
use logic::FilledTileModel;
use props::FilledTileProps;
use style::CLASS;
use tw_macro::assert_component;

/// An occupied command tile. Purely presentational: it draws the ability icon (or its
/// text fallback) over a per-kind background fill, themes its accent from the owning
/// unit's race, and — when selected — mounts the `SelectionRing` whose presence turns
/// the tile's own border gold. It knows nothing of hotkeys, focus, or dragging;
/// `GridEditorTile` layers all interaction on top of this base tile.
#[component]
pub fn FilledTile(props: FilledTileProps) -> Element {
    let FilledTileModel {
        ability_active,
        command_active,
        selected,
        is_dragging_source,
        is_drag_over,
        icon_source,
        icon_alt,
        label_text,
    } = FilledTileModel::from(props);
    rsx! {
        div {
            class: CLASS,
            AbilityFill { active: ability_active }
            CommandFill { active: command_active }
            SelectionRing { selected }
            TileIcon { src: icon_source, alt: icon_alt }
            TileLabel { text: label_text }
            DraggingSourceGhost { active: is_dragging_source }
            DragOverRing { active: is_drag_over }
        }
    }
}

assert_component!(FilledTile);
