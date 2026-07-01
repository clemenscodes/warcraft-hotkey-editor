use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ResolvePlanCountsProps {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
}
