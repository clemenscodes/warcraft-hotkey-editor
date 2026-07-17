pub mod components;
mod model;
mod view;

pub use view::MobileCategoryTabsView;
mod style;

use crate::components::app::components::shell::components::shared::category_scroll::unit_kind_key;
use components::mobile_category_tab::MobileCategoryTab;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

use model::MobileCategoryTabsModel;

#[component]
pub fn MobileCategoryTabs(props: MobileCategoryTabsModel) -> Element {
    rsx! {
        nav {
            class: CLASS,
            role: "tablist",
            aria_label: "Unit categories",
            for kind in props.tabs {
                MobileCategoryTab {
                    key: "{unit_kind_key(kind)}",
                    kind,
                }
            }
        }
    }
}

assert_component!(MobileCategoryTabs);
