mod props;
mod style;
use dioxus::prelude::*;
pub use props::CarrierCardNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CarrierCardName);
#[component]
pub fn CarrierCardName(props: CarrierCardNameProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
