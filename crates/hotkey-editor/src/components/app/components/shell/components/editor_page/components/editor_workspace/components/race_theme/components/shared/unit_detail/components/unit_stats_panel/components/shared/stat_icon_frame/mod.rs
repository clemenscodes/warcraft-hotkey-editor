pub mod components;
mod model;
mod view;

pub use view::StatIconFrameView;
mod style;

use components::stat_icon_img::StatIconImg;
use dioxus::prelude::*;
use model::StatIconFrameModel;
use style::CLASS;
use tw_macro::assert_component;

/// The square icon frame at the top of an icon-bearing stat column.
#[component]
pub fn StatIconFrame(props: StatIconFrameModel) -> Element {
    let src = props.src;
    let alt = props.alt;
    rsx! {
        div {
            class: CLASS,
            StatIconImg {
                src,
                alt,
            }
        }
    }
}

assert_component!(StatIconFrame);
