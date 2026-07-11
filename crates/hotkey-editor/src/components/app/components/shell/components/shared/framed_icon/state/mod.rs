use super::icon_radius::IconRadius;
use super::model::FramedIconModel;

/// The resolved look of a framed icon: the radius token folded together with
/// whether it lifts to a hover glow or draws the empty-placeholder fill. Derived
/// from the public axes of [`FramedIconModel`]. The mapping reflects how the app
/// actually uses the frame — the tile radius appears both plain and glowing, the
/// card radius always glows, the control radius never does, and the hairline radius
/// is only ever the placeholder square — so the whole reachable look space is these
/// five variants and no more.
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
