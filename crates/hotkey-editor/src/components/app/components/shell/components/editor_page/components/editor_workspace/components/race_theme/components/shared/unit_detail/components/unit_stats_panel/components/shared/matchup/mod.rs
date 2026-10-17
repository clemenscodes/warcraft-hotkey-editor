pub mod components;
mod model;
mod view;

pub use view::MatchupView;
mod state;
mod subject;

use components::neutral_matchup_host::NeutralMatchupHost;
use components::strong_matchup_host::StrongMatchupHost;
use components::weak_matchup_host::WeakMatchupHost;
use dioxus::prelude::*;
use model::MatchupModel;
pub use state::MatchupStrength;
pub use subject::MatchupSubject;
use tw_macro::assert_component;

#[component]
pub fn Matchup(props: MatchupModel) -> Element {
    let subject = props.subject;
    let multiplier = props.multiplier;
    let title = props.title;
    match props.strength {
        MatchupStrength::Strong => rsx! {
            StrongMatchupHost {
                subject,
                multiplier,
                title,
            }
        },
        MatchupStrength::Weak => rsx! {
            WeakMatchupHost {
                subject,
                multiplier,
                title,
            }
        },
        MatchupStrength::Neutral => rsx! {
            NeutralMatchupHost {
                subject,
                multiplier,
                title,
            }
        },
    }
}

assert_component!(Matchup);
