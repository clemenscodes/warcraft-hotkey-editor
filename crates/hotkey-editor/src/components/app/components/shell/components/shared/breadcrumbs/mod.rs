mod breadcrumb_view;
pub mod components;
mod model;
mod view;

pub use view::BreadcrumbsView;
mod style;

pub use breadcrumb_view::BreadcrumbView;

use components::breadcrumb::Breadcrumb;
use components::breadcrumb_separator::BreadcrumbSeparator;
use dioxus::prelude::*;
use model::BreadcrumbsModel;
use style::CLASS;
use tw_macro::assert_component;

/// The shared breadcrumb bar, reused by the collisions and resolve pages. Purely
/// presentational: it renders the prepared tabs its owning page hands it, with a
/// separator between each. The page builds the tabs (labels, counts, active flags,
/// navigation handlers) and names the bar via `aria_label`.
#[component]
pub fn Breadcrumbs(props: BreadcrumbsModel) -> Element {
    let breadcrumbs = props.breadcrumbs;
    let aria_label = props.aria_label;
    rsx! {
        nav {
            class: CLASS,
            aria_label,
            for (index, breadcrumb) in breadcrumbs.into_iter().enumerate() {
                if index > 0 {
                    BreadcrumbSeparator {}
                }
                Breadcrumb {
                    label: breadcrumb.label,
                    count: breadcrumb.count,
                    active: breadcrumb.active,
                    onclick: breadcrumb.onclick,
                }
            }
        }
    }
}

assert_component!(Breadcrumbs);
