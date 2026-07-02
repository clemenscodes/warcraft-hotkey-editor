use dioxus::prelude::*;

/// One cell of the resolve mini grid; highlighted (and holding the ability icon)
/// when an ability lands on it.
#[derive(Props, Clone, PartialEq)]
pub struct MiniCellProps {
    pub has_placement: bool,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
}
