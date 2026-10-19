mod model;
mod view;

pub use view::SystemHotkeysSectionIntroView;
mod style;

use dioxus::prelude::*;
use model::SystemHotkeysSectionIntroModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SystemHotkeysSectionIntro(props: SystemHotkeysSectionIntroModel) -> Element {
    let text = props.text;
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(SystemHotkeysSectionIntro);
