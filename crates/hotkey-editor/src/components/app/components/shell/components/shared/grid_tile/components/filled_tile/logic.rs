use super::super::super::GridTileState;
use super::props::FilledTileProps;

/// The shaped inputs each child of a filled tile needs, derived once from the tile's
/// state. It selects the fill (ability vs command), whether the selection ring mounts,
/// and splits the icon/label into the icon source, its alt text and the text fallback —
/// so the body only binds these fields onto its children and never computes.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(super) struct FilledTileModel {
    pub ability_active: bool,
    pub command_active: bool,
    pub selected: bool,
    pub is_dragging_source: bool,
    pub is_drag_over: bool,
    pub icon_source: Option<String>,
    pub icon_alt: String,
    pub label_text: Option<String>,
}

impl From<FilledTileProps> for FilledTileModel {
    fn from(props: FilledTileProps) -> Self {
        let ability_active = matches!(props.state, GridTileState::Filled | GridTileState::Selected);
        let command_active = matches!(props.state, GridTileState::Command);
        let selected = matches!(props.state, GridTileState::Selected);
        let is_dragging_source = props.is_dragging_source;
        let is_drag_over = props.is_drag_over;
        let label = props.label;
        let icon_alt = label.clone();
        let has_icon = props.icon.is_some();
        let label_text = if has_icon { None } else { Some(label) };
        let icon_source = props.icon;
        Self {
            ability_active,
            command_active,
            selected,
            is_dragging_source,
            is_drag_over,
            icon_source,
            icon_alt,
            label_text,
        }
    }
}
