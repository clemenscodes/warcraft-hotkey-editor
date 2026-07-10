pub mod components;
mod props;
mod style;

use components::info_filename::InfoFilename;
use components::info_intro::{InfoIntro, InfoIntroProps};
use components::info_warning::{InfoWarning, InfoWarningProps};
use dioxus::prelude::*;
pub use props::InfoContentProps;
use style::CLASS;
use tw_macro::assert_component;

/// Every info dialog's centered instruction block: the intro line, the filename
/// chip, and the optional warning callout.
#[component]
pub fn InfoContent(props: InfoContentProps) -> Element {
    let intro = InfoIntroProps::from(&props);
    let warning = InfoWarningProps::from(&props);
    rsx! {
        div { class: CLASS,
            InfoIntro { ..intro }
            InfoFilename {}
            InfoWarning { ..warning }
        }
    }
}

assert_component!(InfoContent);
