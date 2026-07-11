pub mod components;
mod model;
pub mod presentation;
mod view;

pub use view::ResolvePageView;

mod style;

use crate::components::app::components::shell::components::shared::breadcrumbs::Breadcrumbs;
use components::clear_state::ClearState;
use components::empty_state::EmptyState;
use components::plan_body::PlanBody;
use components::plan_header::PlanHeader;
use dioxus::prelude::*;
use model::ResolvePageModel;
use presentation::{ResolvePagePresentation, ResolvePlanPresentation, use_resolve_page};
use style::CLASS;
use tw_macro::assert_component;

/// The Resolve page: a transparent preview of the cascade plan — every move the
/// algorithm would make and any unresolved abilities — with an Apply button that
/// runs the cascade. Shows an upload prompt with no file and an all-clear state
/// when there is nothing to resolve. The hook shapes the state's data; the body
/// only places it.
#[component]
pub fn ResolvePage(props: ResolvePageModel) -> Element {
    let plan = match use_resolve_page(&props) {
        ResolvePagePresentation::NoFile => {
            return rsx! {
                EmptyState {}
            };
        }
        ResolvePagePresentation::Clear => {
            return rsx! {
                ClearState {}
            };
        }
        ResolvePagePresentation::Plan(plan) => *plan,
    };
    let ResolvePlanPresentation {
        moves_text,
        unresolved_count,
        running,
        on_apply,
        breadcrumbs,
        section,
        unresolved,
    } = plan;
    rsx! {
        section {
            class: CLASS,
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
            PlanBody {
                section,
                unresolved,
            }
        }
    }
}

assert_component!(ResolvePage);
