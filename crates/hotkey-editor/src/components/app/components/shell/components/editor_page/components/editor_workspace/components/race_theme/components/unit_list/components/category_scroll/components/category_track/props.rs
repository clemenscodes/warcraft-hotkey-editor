use super::components::unit_category_section::UnitCategorySectionProps;
use dioxus::prelude::*;

/// The track's props: the finished props for each category section, in display order.
#[derive(Props, Clone, PartialEq)]
pub struct CategoryTrackProps {
    pub sections: Vec<UnitCategorySectionProps>,
}
