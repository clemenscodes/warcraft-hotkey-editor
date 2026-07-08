mod props;
mod style;

use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeading, GoldHeadingProps,
};
use dioxus::prelude::*;
pub use props::GridHeadingProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(GridHeading);

#[component]
pub fn GridHeading(props: GridHeadingProps) -> Element {
    let heading = GoldHeadingProps::from(&props);
    rsx! {
        h3 {
            class: CLASS,
            GoldHeading { ..heading }
        }
    }
}
