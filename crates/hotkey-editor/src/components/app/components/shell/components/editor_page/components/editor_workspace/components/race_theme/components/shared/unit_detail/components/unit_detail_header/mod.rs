pub mod components;
mod logic;
mod props;
mod style;

use components::unit_detail_title::{UnitDetailTitle, UnitDetailTitleProps};
use components::unit_portrait::{UnitPortrait, UnitPortraitProps};
use dioxus::prelude::*;
pub use props::UnitDetailHeaderProps;
use style::CLASS;
use tw_macro::assert_component;

/// The unit detail header: the portrait beside the title column (name, id, and the
/// optional hero level picker).
#[component]
pub fn UnitDetailHeader(props: UnitDetailHeaderProps) -> Element {
    let portrait = UnitPortraitProps::from(&props);
    let title = UnitDetailTitleProps::from(&props);
    rsx! {
        header {
            class: CLASS,
            UnitPortrait { ..portrait }
            UnitDetailTitle { ..title }
        }
    }
}

assert_component!(UnitDetailHeader);
