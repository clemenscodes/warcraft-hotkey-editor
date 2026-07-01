pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::unit_detail_title::{UnitDetailTitle, UnitDetailTitleProps};
use components::unit_portrait::{UnitPortrait, UnitPortraitProps};
use dioxus::prelude::*;
pub use props::UnitDetailHeaderProps;
use style::CLASS;
assert_component!(UnitDetailHeader);

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
