use super::components::grid::{GridProps, GridTileKind};
use dioxus::prelude::*;

/// A heading stacked above a grid. Purely presentational and generic over the
/// [`GridTileKind`] its grid lays out: it is nothing but a caption plus the grid's
/// own props, passed straight through. It has no behavior of its own and nothing
/// to do with editing — the `GridEditor` builds these props with interactive
/// tiles, the templates preview with read-only tiles; either way `HeadedGrid` just
/// renders.
#[derive(Props, Clone, PartialEq)]
pub struct HeadedGridProps<B: GridTileKind> {
    pub heading: &'static str,
    pub grid: GridProps<B>,
}
