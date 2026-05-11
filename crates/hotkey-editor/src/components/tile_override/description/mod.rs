use dioxus::prelude::*;

/// Renders the primary ubertip / tip text block for an ability or upgrade.
/// Each line is a separate `<p>` so pre-wrap spacing is preserved.
#[derive(Props, Clone, PartialEq)]
pub(crate) struct AbilityDescriptionProps {
    pub(crate) description_lines: Vec<String>,
}

#[component]
pub(crate) fn AbilityDescription(props: AbilityDescriptionProps) -> Element {
    let description_lines = props.description_lines;
    rsx! {
        div {
            class: "flex-1 min-h-0 overflow-y-auto flex flex-col gap-[0.4rem] py-[0.85rem] px-4 bg-[rgba(8,18,35,0.35)] [border-left:2px_solid_#ffce63] rounded text-warcraft-text-secondary text-[1.55rem] leading-[1.55] max-[1099px]:text-[13px] max-[1099px]:leading-[1.35] max-[1099px]:flex-none max-[1099px]:overflow-visible max-[1099px]:max-h-none",
            for description_line in description_lines {
                p { class: "m-0 whitespace-pre-wrap", {description_line} }
            }
        }
    }
}
