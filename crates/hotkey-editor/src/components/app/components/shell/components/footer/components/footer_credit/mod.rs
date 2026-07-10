pub mod components;
mod props;
mod style;

use components::footer_heart::{FooterHeart, FooterHeartProps};
use dioxus::prelude::*;
pub use props::FooterCreditProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterCredit(props: FooterCreditProps) -> Element {
    let heart = FooterHeartProps::from(&props);
    let lead = props.lead;
    let tail = props.tail;
    rsx! {
        span { class: CLASS,
            {lead}
            FooterHeart { ..heart }
            {tail}
        }
    }
}

assert_component!(FooterCredit);
