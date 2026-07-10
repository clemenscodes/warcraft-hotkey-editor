pub mod components;
mod logic;
mod props;
mod state;
mod subject;

use components::neutral_matchup::{NeutralMatchup, NeutralMatchupProps};
use components::strong_matchup::{StrongMatchup, StrongMatchupProps};
use components::weak_matchup::{WeakMatchup, WeakMatchupProps};
use dioxus::prelude::*;
pub use props::MatchupProps;
pub use state::MatchupStrength;
pub use subject::MatchupSubject;
use tw_macro::assert_component;

/// One matchup cell. A dispatcher: from the matchup strength it renders the strong,
/// weak, or neutral cell — each owns its own tint and publishes the `--matchup-color`
/// its value reads, so there is no `data-matchup` attribute.
#[component]
pub fn Matchup(props: MatchupProps) -> Element {
    match props.strength {
        MatchupStrength::Strong => {
            let cell = StrongMatchupProps::from(&props);
            rsx! {
                StrongMatchup { ..cell }
            }
        }
        MatchupStrength::Weak => {
            let cell = WeakMatchupProps::from(&props);
            rsx! {
                WeakMatchup { ..cell }
            }
        }
        MatchupStrength::Neutral => {
            let cell = NeutralMatchupProps::from(&props);
            rsx! {
                NeutralMatchup { ..cell }
            }
        }
    }
}

assert_component!(Matchup);
