use super::components::category_track::CategoryTrackProps;
use super::components::category_track::components::unit_category_section::UnitCategorySectionProps;
use dioxus::prelude::*;

/// The scroll region's props: the finished props for each category section, in
/// display order. Passed straight through to the inner track.
#[derive(Props, Clone, PartialEq)]
pub struct CategoryScrollProps {
    pub sections: Vec<UnitCategorySectionProps>,
}

impl From<&CategoryScrollProps> for CategoryTrackProps {
    fn from(props: &CategoryScrollProps) -> Self {
        let sections = props.sections.clone();
        Self { sections }
    }
}
