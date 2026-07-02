use dioxus::prelude::*;

/// One hero attribute row: its name, current value, per-level gain, and whether it
/// is the hero's primary attribute (which highlights it gold).
#[derive(Props, Clone, PartialEq)]
pub struct AttributeRowProps {
    pub label: &'static str,
    pub value: u32,
    pub per_level: f32,
    pub is_primary: bool,
}
