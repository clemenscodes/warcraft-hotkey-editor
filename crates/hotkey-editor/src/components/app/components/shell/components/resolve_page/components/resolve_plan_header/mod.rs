pub mod components;
mod model;
mod view;

pub use view::ResolvePlanHeaderView;

use crate::components::app::components::shell::components::shared::breadcrumbs::Breadcrumbs;
use components::plan_header::PlanHeader;
use dioxus::prelude::*;
use model::ResolvePlanHeaderModel;
use tw_macro::assert_component;

#[component]
pub fn ResolvePlanHeader(props: ResolvePlanHeaderModel) -> Element {
    let moves_text = props.moves_text;
    let unresolved_count = props.unresolved_count;
    let running = props.running;
    let on_apply = props.on_apply;
    let breadcrumbs = props.breadcrumbs;
    rsx! {
        PlanHeader {
            moves_text,
            unresolved_count,
            running,
            on_apply,
        }
        Breadcrumbs {
            breadcrumbs,
            aria_label: "Move categories",
        }
    }
}

assert_component!(ResolvePlanHeader);
