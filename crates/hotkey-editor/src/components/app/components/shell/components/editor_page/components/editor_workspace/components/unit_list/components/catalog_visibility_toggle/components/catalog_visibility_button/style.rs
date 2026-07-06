use tw_macro::tw;
// One button of the catalog-visibility toggle (No abilities / All variants). A
// smaller bronze pill than the search-field toggle; gold when active. Height is set
// by the group's `[&>button]` rule.

classes! {
    base: tw![
        "flex-1",
        "px-3",
        "whitespace-nowrap",
        "bg-panel-gold-resting",
        "border",
        "border-warcraft-gold-border",
        "rounded-[8px]",
        "text-warcraft-text-secondary",
        "text-[1.1rem]",
        "uppercase",
        "tracking-[0.08em]",
        "text-shadow-drop",
        "transition-[border-color,color,box-shadow]",
        "duration-150",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:focus-ring",
        "data-[active=true]:bg-panel-gold-active",
        "data-[active=true]:border-warcraft-gold",
        "data-[active=true]:text-warcraft-gold",
        "data-[active=true]:shadow-glow-12",
    ],
    mobile: tw![
        "mobile:text-[0.85rem]",
        "mobile:px-[0.5rem]",
    ],
    tablet: tw![
        "tablet:text-[clamp(0.8rem,0.45vw+0.55rem,1.05rem)]",
        "tablet:px-2",
    ],
    laptop: tw![
        "laptop:text-[clamp(0.8rem,0.45vw+0.55rem,1.05rem)]",
        "laptop:px-2",
    ],
    desktop: tw![
        "desktop:text-[clamp(0.8rem,0.45vw+0.55rem,1.05rem)]",
        "desktop:px-2",
    ],
}
