pub mod components;
mod props;
mod style;
use components::breadcrumb::Breadcrumb;
use components::breadcrumb_separator::BreadcrumbSeparator;
use dioxus::prelude::*;
pub use props::BreadcrumbsProps;
use style::CLASS;
use tw_macro::assert_component;

/// The shared breadcrumb bar, reused by the collisions and resolve pages. Purely
/// presentational: it renders the prepared tabs its owning page hands it, with a
/// separator between each. The page builds the tabs (labels, counts, active flags,
/// navigation handlers) and names the bar via `aria_label`.
#[component]
pub fn Breadcrumbs(props: BreadcrumbsProps) -> Element {
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
                Breadcrumb { ..breadcrumb }
            }
        }
    }
}

assert_component!(Breadcrumbs);
