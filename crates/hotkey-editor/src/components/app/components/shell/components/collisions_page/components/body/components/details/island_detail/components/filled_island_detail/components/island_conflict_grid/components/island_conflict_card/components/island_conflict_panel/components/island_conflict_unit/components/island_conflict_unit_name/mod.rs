mod props;
mod style;
use dioxus::prelude::*;
pub use props::IslandConflictUnitNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(IslandConflictUnitName);
#[component]
pub fn IslandConflictUnitName(props: IslandConflictUnitNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
