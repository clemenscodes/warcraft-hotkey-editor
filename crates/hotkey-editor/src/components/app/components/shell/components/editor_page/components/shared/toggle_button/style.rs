use tw_macro::tw;
// The one gold pill — one identical look for every toggle (mode, search-field,
// catalog-visibility). There is deliberately no size variant: these buttons must never
// differ from each other. The look (text, padding, radius, border, active gold) is
// fixed and uniform here; the parent owns only the box the button fills (`flex-1` in
// its row/column, and the group's own `[&>button]:min-h`). Text scales by viewport
// band, identically for all users of this component.

classes! {
    base: tw![
        "flex-1",
        "px-6",
        "bg-panel-gold-resting",
        "border",
        "border-warcraft-gold-border",
        "rounded-panel",
        "text-warcraft-text-secondary",
        "text-xl",
        "uppercase",
        "tracking-[0.08em]",
        "whitespace-nowrap",
        "text-shadow-drop",
        "cursor-pointer",
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
    mobile: tw!["mobile:text-base", "mobile:px-[0.6rem]"],
    tablet: tw!["tablet:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)]", "tablet:px-4"],
    laptop: tw!["laptop:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)]", "laptop:px-4"],
    desktop: tw!["desktop:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)]", "desktop:px-4"],
}
