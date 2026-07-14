pub mod components;
mod model;
mod view;

pub use view::PlanHeaderView;
mod style;
use components::apply_button::ApplyButton;
use components::plan_summary::PlanSummary;
use dioxus::prelude::*;
use model::PlanHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PlanHeader(props: PlanHeaderModel) -> Element {
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

assert_component!(PlanHeader);
