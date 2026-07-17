mod model;
mod view;

pub use view::RaceChipView;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButton;
use dioxus::prelude::*;
use model::RaceChipModel;
use tw_macro::assert_component;

/// A race filter is a toggle like every other filter in this dialog, so it wears
/// the shared [`ToggleButton`] rather than a look of its own. The only thing that
/// varies is the colour, and that arrives as `--race-color` from the theme above.
#[component]
pub fn RaceChip(props: RaceChipModel) -> Element {
    let label = props.label;
    let active = props.active;
    let onclick = props.on_pick;
    rsx! {
        ToggleButton {
            label,
            active,
            onclick,
        }
    }
}

assert_component!(RaceChip);
