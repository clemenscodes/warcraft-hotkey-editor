mod frame;
mod model;
mod style;
mod view;

pub use view::WarcraftPageView;

use browser_kit::frame::Render;
use dioxus::prelude::*;
use dioxus_kit::frame::Page;
use frame::WarcraftPageFrame;
use model::WarcraftPageModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn WarcraftPage<Header: Render<Output = Element>, Body: Render<Output = Element>>(
    props: WarcraftPageModel<Header, Body>,
) -> Element {
    let header = props.header;
    let body = props.body;
    let frame = WarcraftPageFrame { header, body };
    rsx! {
        Page {
            class: CLASS,
            frame,
        }
    }
}

assert_component!(WarcraftPage);
