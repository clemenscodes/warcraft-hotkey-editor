use tw_macro::tw;

// The collision card's button surface: the resting look and interior shared by every
// selectable entity card — the panel fill, the hairline blue border, the rounded
// tile, the hover and keyboard-focus treatment, the selected `bg-panel-blue`, and the
// mobile/tablet carousel interior — written here as its own values (no shared wrapper
// swallows the body). `group` is set so a descendant can read
// `group-data-[selected=true]`. The accent is fixed collision gold: a blue hover
// border, and gold border, text, and glow when selected — no race colour and no state
// table.

classes! {
    base: tw![
        "group",
        "flex",
        "items-center",
        "gap-4",
        "p-4",
        "w-full",
        "min-w-0",
        "text-left",
        "text-lg",
        "tracking-snug",
        "border",
        "rounded-tile",
        "transition-all",
        "duration-fast",
        "bg-warcraft-bg-mid/55",
        "border-warcraft-blue-deep",
        "text-warcraft-text-primary",
        "hover:bg-warcraft-blue-deep/70",
        "hover:text-white",
        "hover:border-warcraft-blue",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:bg-warcraft-blue/85",
        "kb-focus:shadow-focus",
        "data-[selected=true]:bg-panel-blue",
        "data-[selected=true]:border-warcraft-gold",
        "data-[selected=true]:text-warcraft-gold",
        "data-[selected=true]:shadow-glow-soft",
    ],
    mobile: tw![
        "mobile:h-full",
        "mobile:py-2",
        "mobile:px-2.5",
        "mobile:gap-2.5",
        "mobile:box-border",
        "mobile:overflow-hidden",
        "mobile:bg-panel-dark",
        "mobile:border-warcraft-blue/60",
    ],
    tablet: tw![
        "tablet:h-full",
        "tablet:py-2",
        "tablet:px-2.5",
        "tablet:gap-2.5",
        "tablet:box-border",
        "tablet:overflow-hidden",
        "tablet:bg-panel-dark",
        "tablet:border-warcraft-blue/60",
    ],
}
