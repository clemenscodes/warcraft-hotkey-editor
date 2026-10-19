pub mod components;
mod model;
mod view;

pub use view::ResolveSectionNavView;
mod style;

use components::resolve_section_tab::ResolveSectionTab;
use dioxus::prelude::*;
use model::ResolveSectionNavModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolveSectionNav(props: ResolveSectionNavModel) -> Element {
    let breadcrumbs = props.breadcrumbs;
    rsx! {
        nav {
            class: CLASS,
            aria_label: "Move categories",
            for breadcrumb in breadcrumbs {
                ResolveSectionTab {
                    key: "{breadcrumb.label}",
                    label: breadcrumb.label,
                    count: breadcrumb.count,
                    active: breadcrumb.active,
                    onclick: breadcrumb.onclick,
                }
            }
        }
    }
}

assert_component!(ResolveSectionNav);
