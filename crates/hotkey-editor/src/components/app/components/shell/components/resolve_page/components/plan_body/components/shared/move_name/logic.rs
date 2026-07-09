use super::components::link_move_name::LinkMoveNameProps;
use super::components::plain_move_name::PlainMoveNameProps;
use super::props::MoveNameProps;

impl From<&MoveNameProps> for LinkMoveNameProps {
    fn from(props: &MoveNameProps) -> Self {
        let text = props.text.clone();
        Self { text }
    }
}

impl From<&MoveNameProps> for PlainMoveNameProps {
    fn from(props: &MoveNameProps) -> Self {
        let text = props.text.clone();
        Self { text }
    }
}
