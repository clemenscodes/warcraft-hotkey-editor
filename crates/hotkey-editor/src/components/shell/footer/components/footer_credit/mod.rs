pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

use components::footer_heart::{FooterHeart, FooterHeartProps};

pub use props::FooterCreditProps;

assert_component!(FooterCredit);

#[component]
pub fn FooterCredit(props: FooterCreditProps) -> Element {
    let heart = FooterHeartProps::from(&props);
    let lead = props.lead;
    let tail = props.tail;
    rsx! {
        span {
            class: CLASS,
            {lead}
            FooterHeart { ..heart }
            {tail}
        }
    }
}
