pub mod components;
mod props;

use components::active_breadcrumb::ActiveBreadcrumb;
use components::idle_breadcrumb::IdleBreadcrumb;
use dioxus::prelude::*;
use props::BreadcrumbProps;
use tw_macro::assert_component;

/// A single breadcrumb tab. A pure dispatcher: from whether it is the active tab it
/// renders `ActiveBreadcrumb` xor `IdleBreadcrumb`. Each owns its `<button>`, writes its
/// full look, and publishes the `--count-opacity` its count reads — no `data-active`, the
/// look follows the component.
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    match props.active {
        true => {
            let label = props.label.clone();
            let count = props.count;
            let onclick = props.onclick;
            rsx! {
                ActiveBreadcrumb { label, count, onclick }
            }
        }
        false => {
            let label = props.label.clone();
            let count = props.count;
            let onclick = props.onclick;
            rsx! {
                IdleBreadcrumb { label, count, onclick }
            }
        }
    }
}

assert_component!(Breadcrumb);
