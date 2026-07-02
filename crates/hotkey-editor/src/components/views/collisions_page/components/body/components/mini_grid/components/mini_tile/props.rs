use dioxus::prelude::*;

/// One tile of the mini grid; marked when it is the highlighted coordinate's tile.
#[derive(Props, Clone, PartialEq)]
pub struct MiniTileProps {
    pub is_highlighted: bool,
}
