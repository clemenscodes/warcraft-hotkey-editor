pub mod components;
mod props;
mod style;

use components::stat_icon_img::{StatIconImg, StatIconImgProps};
use dioxus::prelude::*;
pub use props::StatIconFrameProps;
use style::CLASS;
use tw_macro::assert_component;
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
