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

/// The reusable conflict-panel card: the shared tinted, bordered conflict surface over the
/// headless `Card` frame primitive. Both the hotkey/unit-position conflict panel and the
/// island conflict panel compose this, supplying only their body region; this owns the surface
/// look (its single `CLASS`, which the headless `Card` applies to its `section` via `class:`).
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
