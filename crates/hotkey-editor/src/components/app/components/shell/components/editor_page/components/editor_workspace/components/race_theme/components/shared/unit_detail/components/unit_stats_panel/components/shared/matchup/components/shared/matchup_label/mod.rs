mod props;
mod style;

use dioxus::prelude::*;
pub use props::MatchupLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MatchupLabel);

/// A matchup cell's label: the attack or defense type it names, rendered through its
/// own `Display`.
#[component]
pub fn MatchupLabel(props: MatchupLabelProps) -> Element {
    let subject = props.subject;
    rsx! {
        span {
            class: CLASS,
            "{subject}"
        }
    }
}
