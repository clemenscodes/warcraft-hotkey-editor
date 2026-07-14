use super::icon_radius::IconRadius;
use super::model::FramedIconModel;

#[derive(Clone, Copy, PartialEq)]
pub enum FramedIconStyle {
    TilePlain,
    TileGlow,
    ControlPlain,
    CardGlow,
    Placeholder,
}

impl From<&FramedIconModel> for FramedIconStyle {
    fn from(props: &FramedIconModel) -> Self {
        if props.placeholder {
            return Self::Placeholder;
        }
        let hover_glow = props.hover_glow;
        match props.radius {
            IconRadius::Tile if hover_glow => Self::TileGlow,
            IconRadius::Tile => Self::TilePlain,
            IconRadius::Control => Self::ControlPlain,
            IconRadius::Card => Self::CardGlow,
            IconRadius::Hairline => Self::Placeholder,
        }
    }
}
