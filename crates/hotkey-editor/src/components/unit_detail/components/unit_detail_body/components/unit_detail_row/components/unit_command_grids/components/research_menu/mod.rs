mod props;

use crate::components::grid_editors::research_grid_editor::ResearchGridEditor;
use dioxus::prelude::*;
pub use props::ResearchMenuProps;

/// The unit's research menu, when it has one; renders nothing otherwise.
#[component]
pub fn ResearchMenu(props: ResearchMenuProps) -> Element {
    let Some(config) = props.config else {
        return rsx! {};
    };
    rsx! {
        ResearchGridEditor { ..config }
    }
}
