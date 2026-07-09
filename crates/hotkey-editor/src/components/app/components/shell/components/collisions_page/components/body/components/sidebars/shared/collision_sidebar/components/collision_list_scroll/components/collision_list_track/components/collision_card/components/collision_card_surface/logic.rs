use super::components::idle_collision_card_surface::IdleCollisionCardSurfaceProps;
use super::components::selected_collision_card_surface::SelectedCollisionCardSurfaceProps;
use super::props::CollisionCardSurfaceProps;

impl From<&CollisionCardSurfaceProps> for SelectedCollisionCardSurfaceProps {
    fn from(props: &CollisionCardSurfaceProps) -> Self {
        let onclick = props.onclick;
        let count = props.count;
        let content = props.content.clone();
        Self {
            onclick,
            count,
            content,
        }
    }
}

impl From<&CollisionCardSurfaceProps> for IdleCollisionCardSurfaceProps {
    fn from(props: &CollisionCardSurfaceProps) -> Self {
        let onclick = props.onclick;
        let count = props.count;
        let content = props.content.clone();
        Self {
            onclick,
            count,
            content,
        }
    }
}
