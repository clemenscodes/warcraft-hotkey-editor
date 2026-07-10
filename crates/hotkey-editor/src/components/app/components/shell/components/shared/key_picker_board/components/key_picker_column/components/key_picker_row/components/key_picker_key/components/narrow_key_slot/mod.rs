mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::{
    ColorKey, ColorKeyProps,
};
use dioxus::prelude::*;
pub use props::NarrowKeySlotProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(NarrowKeySlot);

/// The sizing box for a standard-width picker key. A component owns its look; its
/// parent owns its size — so the key's width lives here, and the color leaf inside
/// fills the box. Standard caps (single letters, digits) get this narrow width.
#[component]
pub fn NarrowKeySlot(props: NarrowKeySlotProps) -> Element {
    let color = ColorKeyProps::from(&props);
    rsx! {
        div { class: CLASS,
            ColorKey { ..color }
        }
    }
}
