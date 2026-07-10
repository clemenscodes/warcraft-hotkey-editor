use super::super::shared::slot_contents::SlotContentsProps;
use dioxus::prelude::*;

/// The conflict-look slot's props: the already-shaped inner content (caption,
/// bound-key, tooltip, and the `dragging` flag). Built by the dispatcher from
/// `SystemSlotProps`; carrying the child props as data is passing data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictSlotProps {
    pub contents: SlotContentsProps,
}
