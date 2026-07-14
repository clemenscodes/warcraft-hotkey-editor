mod frame;
mod model;
mod style;
mod view;

pub use view::DetailCardView;

use browser_kit::frame::Render;
use dioxus::prelude::*;
use dioxus_kit::frame::Card;
use frame::DetailCardFrame;
use model::DetailCardModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DetailCard<Body: Render<Output = Element>>(props: DetailCardModel<Body>) -> Element {
    let body = props.body;
    let frame = DetailCardFrame { body };
    rsx! {
        Card {
            frame,
            class: CLASS,
        }
    }
}

assert_component!(DetailCard);
