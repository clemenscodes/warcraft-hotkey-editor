pub mod components;
mod model;
mod view;

pub use view::FooterCreditView;
mod style;

use components::footer_heart::FooterHeart;
use dioxus::prelude::*;
use model::FooterCreditModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterCredit(props: FooterCreditModel) -> Element {
    let lead = props.lead;
    let tail = props.tail;
    let heart = props.heart;
    rsx! {
        span { class: CLASS,
            {lead}
            FooterHeart { svg: heart }
            {tail}
        }
    }
}

assert_component!(FooterCredit);
