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

/// The reusable collision-detail card: the shared bordered detail surface over the headless
/// `Card` frame primitive. Every collision-detail pane — filled or empty, across all three
/// conflict kinds — composes this, supplying only its body region; this owns the surface look
/// (its single `CLASS`, which the headless `Card` applies to its `section` via `class:`).
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
