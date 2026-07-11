pub mod components;
mod model;
mod view;

pub use view::MatchupView;
mod state;
mod subject;

use components::neutral_matchup::NeutralMatchup;
use components::strong_matchup::StrongMatchup;
use components::weak_matchup::WeakMatchup;
use dioxus::prelude::*;
use model::MatchupModel;
pub use state::MatchupStrength;
pub use subject::MatchupSubject;
use tw_macro::assert_component;

/// One matchup cell. A dispatcher: from the matchup strength it renders the strong,
/// weak, or neutral cell — each owns its own tint and publishes the `--matchup-color`
/// its value reads, so there is no `data-matchup` attribute.
#[component]
pub fn Matchup(props: MatchupModel) -> Element {
    let subject = props.subject;
    let multiplier = props.multiplier;
    let title = props.title;
    match props.strength {
        MatchupStrength::Strong => rsx! {
            StrongMatchup { subject, multiplier, title }
        },
        MatchupStrength::Weak => rsx! {
            WeakMatchup { subject, multiplier, title }
        },
        MatchupStrength::Neutral => rsx! {
            NeutralMatchup { subject, multiplier, title }
        },
    }
}

assert_component!(Matchup);
