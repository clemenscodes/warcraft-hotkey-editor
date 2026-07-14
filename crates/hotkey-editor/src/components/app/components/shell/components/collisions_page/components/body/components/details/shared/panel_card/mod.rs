mod frame;
mod model;
mod style;
mod view;

pub use view::PanelCardView;

use browser_kit::frame::Render;
use dioxus::prelude::*;
use dioxus_kit::frame::Card;
use frame::PanelCardFrame;
use model::PanelCardModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PanelCard<Body: Render<Output = Element>>(props: PanelCardModel<Body>) -> Element {
    let body = props.body;
    let frame = PanelCardFrame { body };
    rsx! {
        Card {
            frame,
            class: CLASS,
        }
    }
}

assert_component!(PanelCard);
