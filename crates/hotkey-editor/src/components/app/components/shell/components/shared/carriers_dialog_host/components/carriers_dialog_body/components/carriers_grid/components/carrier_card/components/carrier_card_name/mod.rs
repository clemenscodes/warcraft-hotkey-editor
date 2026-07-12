mod model;
mod view;

pub use view::CarrierCardNameView;
mod style;
use dioxus::prelude::*;
use model::CarrierCardNameModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn CarrierCardName(props: CarrierCardNameModel) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}

assert_component!(CarrierCardName);
