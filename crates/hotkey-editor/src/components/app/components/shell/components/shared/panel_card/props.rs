use super::state::PanelCardVariant;
use dioxus::prelude::*;

/// The shared panel-card surface: a bordered, tinted card that wraps its typed
/// children. The variant chooses the padding, alignment, and border accent.
#[derive(Props, Clone, PartialEq)]
pub struct PanelCardProps {
    pub variant: PanelCardVariant,
    pub children: Element,
}
