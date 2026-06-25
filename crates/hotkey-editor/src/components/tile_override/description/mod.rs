use dioxus::prelude::*;

/// Renders the primary ubertip / tip text block for an ability or upgrade.
/// Each line is a separate `<p>` so pre-wrap spacing is preserved.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityDescriptionProps {
    pub description_lines: Vec<String>,
}

#[component]
pub fn AbilityDescription(props: AbilityDescriptionProps) -> Element {
    let description_lines = props.description_lines;
    rsx! {
        div {
            class: "tile-override-description",
            for description_line in description_lines {
                p { {description_line} }
            }
        }
    }
}
