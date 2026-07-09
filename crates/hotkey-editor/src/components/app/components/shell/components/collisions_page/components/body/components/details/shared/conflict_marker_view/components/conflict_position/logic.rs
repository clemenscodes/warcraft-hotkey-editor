use super::components::inline_conflict_position::InlineConflictPositionProps;
use super::components::top_conflict_position::TopConflictPositionProps;
use super::props::ConflictPositionProps;

impl From<&ConflictPositionProps> for TopConflictPositionProps {
    fn from(props: &ConflictPositionProps) -> Self {
        let coordinate = props.coordinate;
        Self { coordinate }
    }
}

impl From<&ConflictPositionProps> for InlineConflictPositionProps {
    fn from(props: &ConflictPositionProps) -> Self {
        let coordinate = props.coordinate;
        Self { coordinate }
    }
}
