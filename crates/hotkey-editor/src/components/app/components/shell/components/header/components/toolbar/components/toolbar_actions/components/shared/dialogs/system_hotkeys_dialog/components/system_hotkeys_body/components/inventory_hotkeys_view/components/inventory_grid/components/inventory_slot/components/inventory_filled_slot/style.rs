use tw_macro::tw;

// The inventory slot host: the focusable, draggable outer cell. It owns the cell's
// size (its mobile/tablet aspect ratio) and the drag/edit interaction, and is the
// named `group/tooltip` positioning context the framed `SystemSlot` it wraps reacts
// to for its hover/keyboard-focus glows and reveals its tooltip against. Its
// `.inventory-filled-slot` identity is what the drag hit-test and follower measure.
// It draws no frame itself — the `SystemSlot` fills it.
classes! {
    base: tw![
        "relative",
        "group/tooltip",
        "[anchor-name:--tooltip-anchor]",
        "[anchor-scope:--tooltip-anchor]",
        "cursor-pointer",
        "touch-none",
        "focus:outline-none",
        "kb-focus:outline-none",
    ],
    mobile: tw![
        "mobile:aspect-[1/0.85]",
        "mobile:min-h-0",
    ],
    tablet: tw![
        "tablet:aspect-[1/0.85]",
        "tablet:min-h-0",
    ],
}
