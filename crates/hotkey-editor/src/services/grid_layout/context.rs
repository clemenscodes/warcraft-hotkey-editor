use crate::services::grid_layout::service::GridLayoutService;
use dioxus::prelude::*;
use warcraft_keybinds::GridLayout;

pub(crate) fn use_grid_layout() -> Signal<GridLayout> {
    use_context()
}

pub(crate) fn use_grid_layout_service() -> GridLayoutService {
    use_context()
}
