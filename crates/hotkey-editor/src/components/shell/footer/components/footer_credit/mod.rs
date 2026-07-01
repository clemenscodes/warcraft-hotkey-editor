pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::footer_heart::{FooterHeart, FooterHeartProps};
use dioxus::prelude::*;
pub use props::FooterCreditProps;
use style::CLASS;
assert_component!(FooterCredit);

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
