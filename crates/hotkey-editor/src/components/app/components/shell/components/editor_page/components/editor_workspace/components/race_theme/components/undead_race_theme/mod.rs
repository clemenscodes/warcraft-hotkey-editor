mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// An invisible marker the race theme mounts while Undead is the active race. It
/// renders nothing the user sees; it carries the `.undead-race-theme` identity so the
/// theme's own style can publish the race colour with `has-[.undead-race-theme]`.
#[component]
pub fn UndeadRaceTheme() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
        }
    }
}

assert_component!(UndeadRaceTheme);
