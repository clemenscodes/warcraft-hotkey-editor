pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::stat_icon_img::{StatIconImg, StatIconImgProps};
use dioxus::prelude::*;
pub use props::StatIconFrameProps;
use style::CLASS;
assert_component!(StatIconFrame);

/// The square icon frame at the top of an icon-bearing stat column.
#[component]
pub fn StatIconFrame(props: StatIconFrameProps) -> Element {
    let icon = StatIconImgProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            StatIconImg { ..icon }
        }
    }
}
