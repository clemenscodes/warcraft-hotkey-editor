mod model;
mod view;

pub use view::ResolveApplyBarView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::resolve_plan_header::components::plan_header::components::apply_button::ApplyButton;
use crate::components::app::components::shell::components::resolve_page::components::resolve_plan_header::components::plan_header::components::plan_summary::PlanSummary;
use dioxus::prelude::*;
use model::ResolveApplyBarModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolveApplyBar(props: ResolveApplyBarModel) -> Element {
    let moves_text = props.moves_text;
    let unresolved_count = props.unresolved_count;
    let running = props.running;
    let on_apply = props.on_apply;
    rsx! {
        header {
            class: CLASS,
            PlanSummary {
                moves_text,
                unresolved_count,
            }
            ApplyButton {
                running,
                onclick: on_apply,
            }
        }
    }
}

assert_component!(ResolveApplyBar);
