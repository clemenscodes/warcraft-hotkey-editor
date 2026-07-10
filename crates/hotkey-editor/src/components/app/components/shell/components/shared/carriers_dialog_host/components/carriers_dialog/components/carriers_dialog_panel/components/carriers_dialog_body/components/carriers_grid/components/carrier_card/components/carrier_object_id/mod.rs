mod props;
mod view;

pub use view::CarrierObjectIdView;
mod style;
use dioxus::prelude::*;
use props::CarrierObjectIdProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn CarrierObjectId(props: CarrierObjectIdProps) -> Element {
    let unit_id = props.unit_id;
    rsx! {
        code {
            class: CLASS,
            {unit_id.value()}
        }
    }
}

assert_component!(CarrierObjectId);
