use super::components::unit_category_heading::UnitCategoryHeadingProps;
use super::props::UnitCategorySectionProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_list::unit_kind_data_attr;
use dioxus::prelude::*;

impl From<&UnitCategorySectionProps> for UnitCategoryHeadingProps {
    fn from(props: &UnitCategorySectionProps) -> Self {
        let label = props.category_label.clone();
        let kind_attr = unit_kind_data_attr(props.category_kind);
        let is_collapsed = props.is_collapsed;
        let category_kind = props.category_kind;
        let mut collapsed_categories = props.collapsed_categories;
        let on_toggle = EventHandler::new(move |_event: MouseEvent| {
            let mut categories = collapsed_categories.write();
            if categories.contains(&category_kind) {
                categories.remove(&category_kind);
            } else {
                categories.insert(category_kind);
            }
        });
        Self {
            label,
            kind_attr,
            is_collapsed,
            on_toggle,
        }
    }
}
