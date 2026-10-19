mod model;
mod view;

pub use view::TemplatePagerCardHostView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::components::templates_dialog_body::components::shared::template_card::TemplateCard;
use dioxus::prelude::*;
use model::TemplatePagerCardHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TemplatePagerCardHost(props: TemplatePagerCardHostModel) -> Element {
    let name = props.name;
    let description = props.description;
    let resolved = props.resolved;
    let on_apply = props.on_apply;
    rsx! {
        div {
            class: CLASS,
            TemplateCard {
                name,
                description,
                resolved,
                on_apply,
            }
        }
    }
}

assert_component!(TemplatePagerCardHost);
