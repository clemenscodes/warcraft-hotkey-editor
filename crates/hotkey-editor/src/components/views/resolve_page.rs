use dioxus::prelude::*;

/// Placeholder for the Resolve top-level page.  The real content
/// (last-run report, preview/journal) is implemented in issues #43 and
/// #46; this stub establishes routing so deep-linking and the header
/// view switcher can land here.
#[component]
pub(crate) fn ResolvePage() -> Element {
    rsx! {
        section {
            class: "resolve-page flex flex-col items-center justify-center \
                    gap-[1.5rem] p-[2.5rem] text-warcraft-text-secondary text-center \
                    [flex:1_1_0] [min-height:0]",
            h2 {
                class: "font-friz-quadrata uppercase tracking-[0.14em] \
                        text-warcraft-gold text-[clamp(20px,2.6vw,32px)] m-0",
                "Resolve"
            }
            p {
                class: "max-w-[640px] m-0 leading-[1.6] text-[clamp(13px,1.4vw,18px)]",
                "The Resolve dialog will live here. It shows the cascade plan, lets you preview the \
                 changes, and applies them to your CustomKeys.txt. Implementation lands in #43 / #46."
            }
        }
    }
}
