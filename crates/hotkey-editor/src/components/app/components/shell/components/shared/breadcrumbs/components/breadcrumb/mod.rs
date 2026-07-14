pub mod components;
mod model;
mod view;

pub use view::BreadcrumbView;

use components::active_breadcrumb::ActiveBreadcrumb;
use components::idle_breadcrumb::IdleBreadcrumb;
use dioxus::prelude::*;
use model::BreadcrumbModel;
use tw_macro::assert_component;

/// A single breadcrumb tab. A pure dispatcher: from whether it is the active tab it
/// renders `ActiveBreadcrumb` xor `IdleBreadcrumb`. Each owns its `<button>`, writes its
/// full look, and publishes the `--count-opacity` its count reads — no `data-active`, the
/// look follows the component.
#[component]
pub fn Breadcrumb(props: BreadcrumbModel) -> Element {
    match props.active {
        true => {
            let label = props.label.clone();
            let count = props.count;
            let onclick = props.onclick;
            rsx! {
                ActiveBreadcrumb {
                    label,
                    count,
                    onclick,
                }
            }
        }
        false => {
            let label = props.label.clone();
            let count = props.count;
            let onclick = props.onclick;
            rsx! {
                IdleBreadcrumb {
                    label,
                    count,
                    onclick,
                }
            }
        }
    }
}

assert_component!(Breadcrumb);
