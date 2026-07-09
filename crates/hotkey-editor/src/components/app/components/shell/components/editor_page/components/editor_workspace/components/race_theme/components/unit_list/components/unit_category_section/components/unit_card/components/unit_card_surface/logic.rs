use super::components::idle_unit_card_surface::IdleUnitCardSurfaceProps;
use super::components::selected_unit_card_surface::SelectedUnitCardSurfaceProps;
use super::props::UnitCardSurfaceProps;

impl From<&UnitCardSurfaceProps> for SelectedUnitCardSurfaceProps {
    fn from(props: &UnitCardSurfaceProps) -> Self {
        let icon_path = props.icon_path.clone();
        let display_name = props.display_name.clone();
        let unit_id = props.unit_id;
        let race = props.race;
        let onclick = props.onclick;
        let onkeydown = props.onkeydown;
        let onmounted = props.onmounted;
        Self {
            icon_path,
            display_name,
            unit_id,
            race,
            onclick,
            onkeydown,
            onmounted,
        }
    }
}

impl From<&UnitCardSurfaceProps> for IdleUnitCardSurfaceProps {
    fn from(props: &UnitCardSurfaceProps) -> Self {
        let icon_path = props.icon_path.clone();
        let display_name = props.display_name.clone();
        let unit_id = props.unit_id;
        let race = props.race;
        let onclick = props.onclick;
        let onkeydown = props.onkeydown;
        let onmounted = props.onmounted;
        Self {
            icon_path,
            display_name,
            unit_id,
            race,
            onclick,
            onkeydown,
            onmounted,
        }
    }
}
