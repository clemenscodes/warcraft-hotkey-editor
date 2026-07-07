use super::state::LayoutTileState;
use tw_macro::tw;

classes! {
    base: tw![
        "size-full",
        "flex",
        "items-center",
        "justify-center",
        "p-0",
        "border-2",
        "rounded-panel",
        "text-5xl",
        "leading-none",
        "uppercase",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "bg-warcraft-gold-dark/75",
        "text-shadow-outline",
        "hover:shadow-glow-8",
        "hover:bg-warcraft-gold/12",
        "focus:outline-none",
        "kb-focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:bg-warcraft-highlight/12",
        "kb-focus:focus-ring",
        "[@media(hover:none)]:kb-focus:border-warcraft-gold",
        "[@media(hover:none)]:kb-focus:bg-warcraft-gold-dark/75",
        "[@media(hover:none)]:kb-focus:[box-shadow:none]",
        "[@media(hover:none)]:kb-focus:text-warcraft-gold",
    ],
    mobile: tw![
        "mobile:text-2xl",
    ],
    tablet: tw![
        "tablet:text-2xl",
    ],
}

states! {
    LayoutTileState,
    Idle => tw![],
    Editing => tw![
        "bg-panel-gold-diag-30",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow-18",
    ],
}
