pub mod components;
mod logic;
mod props;

use components::active_breadcrumb::{ActiveBreadcrumb, ActiveBreadcrumbProps};
use components::idle_breadcrumb::{IdleBreadcrumb, IdleBreadcrumbProps};
use dioxus::prelude::*;
pub use props::BreadcrumbProps;
use tw_macro::assert_component;

/// A single breadcrumb tab. A pure dispatcher: from whether it is the active tab it
/// renders `ActiveBreadcrumb` xor `IdleBreadcrumb`. Each owns its `<button>`, writes its
/// full look, and publishes the `--count-opacity` its count reads — no `data-active`, the
/// look follows the component.
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    match props.active {
        true => {
            let breadcrumb = ActiveBreadcrumbProps::from(&props);
            rsx! {
                ActiveBreadcrumb { ..breadcrumb }
            }
        }
        false => {
            let breadcrumb = IdleBreadcrumbProps::from(&props);
            rsx! {
                IdleBreadcrumb { ..breadcrumb }
            }
        }
    }
}

assert_component!(Breadcrumb);
