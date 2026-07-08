mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::components::grid::Grid;
use dioxus::prelude::*;
pub use props::MiniGridFrameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MiniGridFrame);

/// The shared mini command-grid frame: the chrome around a shrunk read-only `Grid`.
/// It owns the query container, the panel surface, and the tile-scope border/radius
/// overrides, and lays out whatever twelve tiles its page wrapper hands it. The
/// wrapper owns the outer width and corner radius.
#[component]
pub fn MiniGridFrame(props: MiniGridFrameProps) -> Element {
    let grid = props.grid;
    rsx! {
        div {
            class: CLASS,
            Grid { ..grid }
        }
    }
}
