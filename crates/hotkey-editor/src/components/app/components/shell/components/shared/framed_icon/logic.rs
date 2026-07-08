use super::components::card_glow_icon::CardGlowIconProps;
use super::components::control_plain_icon::ControlPlainIconProps;
use super::components::placeholder_icon::PlaceholderIconProps;
use super::components::tile_glow_icon::TileGlowIconProps;
use super::components::tile_plain_icon::TilePlainIconProps;
use super::props::FramedIconProps;

impl From<&FramedIconProps> for TilePlainIconProps {
    fn from(props: &FramedIconProps) -> Self {
        let source = props.src.clone();
        let alt = props.alt.clone();
        Self { source, alt }
    }
}

impl From<&FramedIconProps> for TileGlowIconProps {
    fn from(props: &FramedIconProps) -> Self {
        let source = props.src.clone();
        let alt = props.alt.clone();
        Self { source, alt }
    }
}

impl From<&FramedIconProps> for ControlPlainIconProps {
    fn from(props: &FramedIconProps) -> Self {
        let source = props.src.clone();
        let alt = props.alt.clone();
        Self { source, alt }
    }
}

impl From<&FramedIconProps> for CardGlowIconProps {
    fn from(props: &FramedIconProps) -> Self {
        let source = props.src.clone();
        let alt = props.alt.clone();
        Self { source, alt }
    }
}

impl From<&FramedIconProps> for PlaceholderIconProps {
    fn from(props: &FramedIconProps) -> Self {
        let source = props.src.clone();
        let alt = props.alt.clone();
        Self { source, alt }
    }
}
