pub mod components;
mod model;
mod view;

pub use view::MobileResolveView;
mod style;

use components::resolve_apply_bar::ResolveApplyBar;
use components::resolve_pager::ResolvePager;
use components::resolve_section_nav::ResolveSectionNav;
use dioxus::prelude::*;
use model::MobileResolveModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MobileResolve(props: MobileResolveModel) -> Element {
    let moves_text = props.moves_text;
    let unresolved_count = props.unresolved_count;
    let running = props.running;
    let on_apply = props.on_apply;
    let breadcrumbs = props.breadcrumbs;
    let section = props.section;
    let unresolved = props.unresolved;
    rsx! {
        div {
            class: CLASS,
            ResolveApplyBar {
                moves_text,
                unresolved_count,
                running,
                on_apply,
            }
            ResolveSectionNav {
                breadcrumbs,
            }
            ResolvePager {
                section,
                unresolved,
            }
        }
    }
}

assert_component!(MobileResolve);
