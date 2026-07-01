use super::components::stat_icon_img::StatIconImgProps;
use dioxus::prelude::*;

/// The square frame holding a stat column's icon.
#[derive(Props, Clone, PartialEq)]
pub struct StatIconFrameProps {
    pub src: Asset,
    #[props(into)]
    pub alt: String,
}

impl From<&StatIconFrameProps> for StatIconImgProps {
    fn from(props: &StatIconFrameProps) -> Self {
        let src = props.src;
        let alt = props.alt.clone();
        Self { src, alt }
    }
}
