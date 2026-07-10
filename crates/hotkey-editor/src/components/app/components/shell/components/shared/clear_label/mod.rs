mod props;
mod style;
use dioxus::prelude::*;
use props::ClearLabelProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ClearLabel(props: ClearLabelProps) -> Element {
    let text = props.text;
    rsx! { p { class: CLASS, {text} } }
}

assert_component!(ClearLabel);
