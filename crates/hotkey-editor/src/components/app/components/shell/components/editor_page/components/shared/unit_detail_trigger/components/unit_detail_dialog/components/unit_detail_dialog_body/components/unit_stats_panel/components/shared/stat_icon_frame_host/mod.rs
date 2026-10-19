pub mod components;
mod model;
mod view;

pub use view::StatIconFrameHostView;
mod style;

use components::stat_icon_frame::StatIconFrame;
use dioxus::prelude::*;
use model::StatIconFrameHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn StatIconFrameHost(props: StatIconFrameHostModel) -> Element {
    let src = props.src;
    let alt = props.alt;
    rsx! {
        div {
            class: CLASS,
            StatIconFrame {
                src,
                alt,
            }
        }
    }
}

assert_component!(StatIconFrameHost);
