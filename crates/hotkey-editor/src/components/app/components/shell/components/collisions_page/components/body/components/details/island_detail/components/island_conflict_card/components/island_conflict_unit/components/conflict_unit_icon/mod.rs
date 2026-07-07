pub mod components;
mod props;
mod style;
use components::conflict_unit_image::{ConflictUnitImage, ConflictUnitImageProps};
use dioxus::prelude::*;
pub use props::ConflictUnitIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictUnitIcon);
/// A conflicting unit's portrait. A guarded host: renders nothing when the unit
/// has no icon; otherwise a framed slot whose image fills it.
#[component]
pub fn ConflictUnitIcon(props: ConflictUnitIconProps) -> Element {
    let Some(source) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    let image_props = ConflictUnitImageProps { source, alt };
    rsx! {
        div { class: CLASS,
            ConflictUnitImage { ..image_props }
        }
    }
}
