pub mod components;
mod model;
mod view;

pub use view::MobileCategoryTabsView;
mod style;

use components::mobile_category_tab::MobileCategoryTab;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::unit_kind_key;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

use model::MobileCategoryTabsModel;

/// The mobile/tablet category tab row (hidden on the sidebar): a `tablist` of one
/// `MobileCategoryTab` per category. The tabs arrive already shaped as props.
#[component]
pub fn MobileCategoryTabs(props: MobileCategoryTabsModel) -> Element {
    rsx! {
        nav {
            class: CLASS,
            role: "tablist",
            aria_label: "Unit categories",
            for kind in props.tabs {
                MobileCategoryTab { key: "{unit_kind_key(kind)}", kind }
            }
        }
    }
}

assert_component!(MobileCategoryTabs);
