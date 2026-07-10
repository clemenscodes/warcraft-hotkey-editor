use tw_macro::tw;

// The hero-selection / control-group slot host: the focusable, clickable outer
// button. It owns the cell's size (aspect ratio, min-height, the `compact`
// control-group cell) and interaction, and is the named `group/tooltip` positioning
// context the framed `SystemSlot` it wraps reacts to for its hover/keyboard-focus
// glows and reveals its tooltip against. It draws no frame itself — the button
// chrome is reset so the `SystemSlot` fills it edge to edge.
classes! {
    base: tw![
        "relative",
        "group/tooltip",
        "[anchor-name:--tooltip-anchor]",
        "[anchor-scope:--tooltip-anchor]",
        "appearance-none",
        "border-0",
        "bg-transparent",
        "p-0",
        "cursor-pointer",
        "touch-manipulation",
        "focus:outline-none",
        "kb-focus:outline-none",
        "data-[compact=true]:min-h-44",
    ],
    mobile: tw![
        "mobile:aspect-[1/0.95]",
        "mobile:min-h-19",
        "mobile:data-[compact=true]:min-h-0",
        "mobile:data-[compact=true]:aspect-square",
    ],
    tablet: tw![
        "tablet:aspect-[1/0.95]",
        "tablet:min-h-19",
        "tablet:data-[compact=true]:min-h-0",
        "tablet:data-[compact=true]:aspect-square",
    ],
}
