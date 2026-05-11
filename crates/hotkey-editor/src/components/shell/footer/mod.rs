use dioxus::prelude::*;

const HEART_SVG: &str = r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" focusable="false"><path fill="currentColor" d="M12 21s-7.5-4.6-9.9-9.3C.3 7.9 2.7 4 6.4 4c2 0 3.6 1 4.6 2.3h2C14 5 15.6 4 17.6 4c3.7 0 6.1 3.9 4.3 7.7C19.5 16.4 12 21 12 21Z"/></svg>"##;

const GITHUB_SVG: &str = r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" focusable="false"><path fill="currentColor" d="M12 .5C5.65.5.5 5.65.5 12c0 5.08 3.29 9.39 7.86 10.91.58.11.79-.25.79-.56 0-.27-.01-1-.02-1.96-3.2.7-3.87-1.54-3.87-1.54-.52-1.32-1.27-1.68-1.27-1.68-1.04-.71.08-.7.08-.7 1.15.08 1.76 1.18 1.76 1.18 1.02 1.75 2.69 1.24 3.35.95.1-.74.4-1.24.73-1.53-2.55-.29-5.24-1.28-5.24-5.69 0-1.26.45-2.28 1.18-3.08-.12-.29-.51-1.46.11-3.05 0 0 .97-.31 3.18 1.18a11.07 11.07 0 0 1 5.78 0c2.21-1.49 3.18-1.18 3.18-1.18.62 1.59.23 2.76.11 3.05.74.8 1.18 1.82 1.18 3.08 0 4.42-2.69 5.39-5.25 5.68.41.36.78 1.06.78 2.13 0 1.54-.01 2.78-.01 3.16 0 .31.21.68.8.56C20.21 21.39 23.5 17.08 23.5 12 23.5 5.65 18.35.5 12 .5Z"/></svg>"##;

const REPO_URL: &str = "https://github.com/clemenscodes/warcraft-hotkey-editor";
const LICENSE_URL: &str =
    "https://github.com/clemenscodes/warcraft-hotkey-editor/blob/main/LICENSE";
const DISCLAIMER_URL: &str =
    "https://github.com/clemenscodes/warcraft-hotkey-editor/blob/main/DISCLAIMER.md";

#[component]
pub(crate) fn Footer() -> Element {
    rsx! {
        footer {
            class: "flex-none flex flex-wrap items-center justify-center gap-x-3 gap-y-1 \
                    pt-5 pb-3 \
                    text-sm tracking-wide text-white/60 select-none \
                    max-[1099px]:text-xs max-[1099px]:text-center max-[1099px]:leading-[1.3] \
                    max-[500px]:text-[11px] \
                    max-[1099px]:pt-2 max-[1099px]:px-[max(0.5rem,env(safe-area-inset-left))] \
                    max-[1099px]:pb-[max(0.5rem,env(safe-area-inset-bottom))] \
                    min-[701px]:max-[1099px]:mt-auto \
                    [@media(min-width:701px)_and_(max-width:1099px)_and_(max-height:900px)]:mt-0 \
                    [@media(min-width:1440px)_and_(max-width:2000px)_and_(max-height:1100px)]:!pt-1 \
                    [@media(min-width:1440px)_and_(max-width:2000px)_and_(max-height:1100px)]:!pb-0",
            span {
                class: "flex items-center gap-2",
                "Crafted with"
                span {
                    class: "inline-flex items-center justify-center \
                            w-4 h-4 text-rose-400/90 \
                            drop-shadow-[0_0_4px_rgba(244,114,182,0.35)]",
                    aria_hidden: "true",
                    dangerous_inner_html: HEART_SVG,
                }
                "by Clemens"
            }
            span { class: "text-white/30", aria_hidden: "true", "·" }
            a {
                class: "inline-flex items-center gap-1.5 \
                        text-white/60 hover:text-warcraft-gold \
                        transition-colors",
                href: REPO_URL,
                target: "_blank",
                rel: "noopener noreferrer",
                span {
                    class: "inline-flex items-center justify-center w-4 h-4",
                    aria_hidden: "true",
                    dangerous_inner_html: GITHUB_SVG,
                }
                "Source on GitHub"
            }
            span { class: "text-white/30", aria_hidden: "true", "·" }
            a {
                class: "text-white/60 hover:text-warcraft-gold transition-colors",
                href: LICENSE_URL,
                target: "_blank",
                rel: "noopener noreferrer",
                "AGPL-3.0"
            }
            span { class: "text-white/30", aria_hidden: "true", "·" }
            a {
                class: "text-white/60 hover:text-warcraft-gold transition-colors",
                href: DISCLAIMER_URL,
                target: "_blank",
                rel: "noopener noreferrer",
                "Disclaimer"
            }
            span { class: "text-white/20 w-full text-center text-xs mt-0.5" }
            span {
                class: "w-full text-center text-xs text-white/30",
                "Not affiliated with or endorsed by Blizzard Entertainment. \
                 Warcraft III is a trademark of Blizzard Entertainment, Inc."
            }
        }
    }
}
