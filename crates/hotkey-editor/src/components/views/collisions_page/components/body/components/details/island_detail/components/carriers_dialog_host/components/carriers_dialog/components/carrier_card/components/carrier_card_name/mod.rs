mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CarrierCardNameProps;
use style::CLASS;
assert_component!(CarrierCardName);
#[component]
pub fn CarrierCardName(props: CarrierCardNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
