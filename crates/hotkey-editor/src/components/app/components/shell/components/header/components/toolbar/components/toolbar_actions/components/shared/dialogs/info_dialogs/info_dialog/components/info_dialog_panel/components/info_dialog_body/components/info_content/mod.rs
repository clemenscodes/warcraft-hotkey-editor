pub mod components;
mod model;
mod view;

pub use view::InfoContentView;
mod style;

use components::info_filename::InfoFilename;
use components::info_intro::InfoIntro;
use components::info_warning::InfoWarning;
use dioxus::prelude::*;
use model::InfoContentModel;
use style::CLASS;
use tw_macro::assert_component;

/// Every info dialog's centered instruction block: the intro line, the filename
/// chip, and the optional warning callout.
#[component]
pub fn InfoContent(props: InfoContentModel) -> Element {
    let intro = props.intro;
    let warning = props.warning;
    rsx! {
        div { class: CLASS,
            InfoIntro { intro }
            InfoFilename {}
            InfoWarning { warning }
        }
    }
}

assert_component!(InfoContent);
