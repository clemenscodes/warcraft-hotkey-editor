use super::state::MatchupStrength;
use super::subject::MatchupSubject;
use super::view::MatchupView;
use dioxus::prelude::*;

/// One matchup cell: the defense/attack subject it names, its damage multiplier, the
/// tooltip, and how strong the matchup is (which tints the cell and value). The subject
/// and multiplier stay domain-typed; the leaves render them.
#[derive(Props, Clone, PartialEq)]
pub struct MatchupProps {
    pub subject: MatchupSubject,
    pub multiplier: f32,
    #[props(into)]
    pub title: String,
    #[props(default)]
    pub strength: MatchupStrength,
}

impl From<&MatchupView> for MatchupProps {
    fn from(view: &MatchupView) -> Self {
        let MatchupView {
            subject,
            multiplier,
            title,
            strength,
        } = view.clone();
        Self {
            subject,
            multiplier,
            title,
            strength,
        }
    }
}

impl ddd::Props for MatchupProps {
    type View = MatchupView;
}
