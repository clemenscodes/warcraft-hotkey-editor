use dioxus::prelude::*;

/// Renders the primary ubertip / tip text block for an ability or upgrade.
/// Each line is a separate `<p>` so pre-wrap spacing is preserved.
#[component]
pub(crate) fn AbilityDescription(description_lines: Vec<String>) -> Element {
    rsx! {
        div { class: "tile-override-description",
            for description_line in description_lines.iter() {
                p { class: "tile-override-description-line", "{description_line}" }
            }
        }
    }
}
