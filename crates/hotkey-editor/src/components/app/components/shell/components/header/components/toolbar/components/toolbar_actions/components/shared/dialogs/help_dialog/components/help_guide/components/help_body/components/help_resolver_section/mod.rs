pub mod components;
mod model;
mod view;

pub use view::HelpResolverSectionView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::components::help_body::components::shared::help_section_title::HelpSectionTitle;
use components::help_glossary_columns::HelpGlossaryColumns;
use dioxus::prelude::*;
use model::HelpResolverSectionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpResolverSection(props: HelpResolverSectionModel) -> Element {
    let columns = props.glossary;
    rsx! {
        section {
            class: CLASS,
            HelpSectionTitle {
                title: "Glossary",
            }
            HelpGlossaryColumns {
                columns,
            }
        }
    }
}

assert_component!(HelpResolverSection);
