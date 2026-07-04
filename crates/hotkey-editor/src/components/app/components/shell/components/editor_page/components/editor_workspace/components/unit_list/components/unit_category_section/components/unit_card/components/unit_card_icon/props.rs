use crate::model::icons::IconUrl;
use dioxus::prelude::*;

/// The portrait source (absent for units without an icon) and the alt text.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardIconProps {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
}
