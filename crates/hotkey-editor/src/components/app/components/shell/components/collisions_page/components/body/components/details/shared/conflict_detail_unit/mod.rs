pub mod components;
mod hooks;
mod props;
mod style;
use components::conflict_detail_unit_icon::ConflictDetailUnitIcon;
use dioxus::prelude::*;
use hooks::use_conflict_detail_unit;
pub use props::ConflictDetailUnitProps;
use style::CLASS;
use tw_macro::assert_component;
/// The clickable unit portrait in the detail header. It deep-links into the editor
/// focused on its unit through the navigation read from context.
#[component]
pub fn ConflictDetailUnit(props: ConflictDetailUnitProps) -> Element {
    let model = use_conflict_detail_unit(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick: model.onclick,
            ConflictDetailUnitIcon { ..model.icon }
        }
    }
}

assert_component!(ConflictDetailUnit);
