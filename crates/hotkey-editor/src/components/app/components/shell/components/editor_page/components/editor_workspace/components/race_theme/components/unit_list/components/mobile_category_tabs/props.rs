use super::components::mobile_category_tab::MobileCategoryTabProps;
use dioxus::prelude::*;

/// The tab row's props: the finished props for each category tab, in display order.
#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabsProps {
    pub tabs: Vec<MobileCategoryTabProps>,
}
