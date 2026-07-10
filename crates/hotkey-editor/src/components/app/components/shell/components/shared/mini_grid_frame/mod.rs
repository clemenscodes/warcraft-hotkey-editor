pub mod components;
mod props;
mod style;

use components::mini_grid::MiniGrid;
use dioxus::prelude::*;
use props::MiniGridFrameProps;
use style::CLASS;
use tw_macro::assert_component;

/// The shared mini command-grid frame: the chrome around a shrunk read-only
/// `MiniGrid`. It owns the query container, the panel surface, and the tile-scope
/// border/radius overrides, and lays out whatever twelve tiles its page wrapper
/// hands it. The wrapper owns the outer width and corner radius.
#[component]
pub fn MiniGridFrame(props: MiniGridFrameProps) -> Element {
    let tiles = props.tiles;
    rsx! {
        div {
            class: CLASS,
            MiniGrid { tiles }
        }
    }
}

assert_component!(MiniGridFrame);
