mod props;
mod style;
use dioxus::prelude::*;
pub use props::ClearLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearLabel);
#[component]
pub fn ClearLabel(props: ClearLabelProps) -> Element {
    let text = props.text;
    rsx! { p { class: CLASS, {text} } }
}
