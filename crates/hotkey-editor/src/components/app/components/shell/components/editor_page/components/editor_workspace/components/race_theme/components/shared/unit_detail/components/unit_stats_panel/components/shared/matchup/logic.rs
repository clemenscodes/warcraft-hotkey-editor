use super::components::neutral_matchup::NeutralMatchupProps;
use super::components::shared::matchup_label::MatchupLabelProps;
use super::components::shared::matchup_value::MatchupValueProps;
use super::components::strong_matchup::StrongMatchupProps;
use super::components::weak_matchup::WeakMatchupProps;
use super::props::MatchupProps;

impl From<&MatchupProps> for StrongMatchupProps {
    fn from(props: &MatchupProps) -> Self {
        let label = MatchupLabelProps::from(props);
        let value = MatchupValueProps::from(props);
        let title = props.title.clone();
        Self {
            label,
            value,
            title,
        }
    }
}

impl From<&MatchupProps> for WeakMatchupProps {
    fn from(props: &MatchupProps) -> Self {
        let label = MatchupLabelProps::from(props);
        let value = MatchupValueProps::from(props);
        let title = props.title.clone();
        Self {
            label,
            value,
            title,
        }
    }
}

impl From<&MatchupProps> for NeutralMatchupProps {
    fn from(props: &MatchupProps) -> Self {
        let label = MatchupLabelProps::from(props);
        let value = MatchupValueProps::from(props);
        let title = props.title.clone();
        Self {
            label,
            value,
            title,
        }
    }
}
