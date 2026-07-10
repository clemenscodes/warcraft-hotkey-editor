use tw_macro::tw;

// The framed WC3 slot cell: the gold border-image frame, the caption/key layout, and
// the glow filters. It fills the box its host hands it (`size-full`) and draws the
// frame at that size; the host owns the cell's size (aspect ratio, min-height) and
// interaction. The frame's density (border width, padding, gap, image slice) is
// inherited from the parent size container through the `--slot-frame-*` custom
// properties (the control-group row tightens them); with no container override the
// per-band fallbacks reproduce the regular hero/inventory frame. The hover and
// keyboard-focus glows are keyed off the host's named `group/tooltip`. The conflict
// look adds a danger-red glow overlay (the slot is in a binding conflict).
classes! {
    base: tw![
        "size-full",
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-[var(--slot-frame-gap,calc(var(--spacing)*2))]",
        "px-[var(--slot-frame-pad-x,calc(var(--spacing)*2.5))]",
        "py-[var(--slot-frame-pad-y,calc(var(--spacing)*3.5))]",
        "text-center",
        "select-none",
        "border-solid",
        "border-[length:var(--slot-frame-border,12px)]",
        "bg-panel-dark",
        "[border-image-source:var(--wc3-slot-frame)]",
        "[border-image-slice:var(--slot-frame-slice,12_fill)]",
        "[border-image-repeat:stretch]",
        "transition-[filter]",
        "group-hover/tooltip:filter-[brightness(1.18)_drop-shadow(0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent))]",
        "group-focus-visible/tooltip:filter-[brightness(1.25)_drop-shadow(0_0_10px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent))]",
        "filter-[drop-shadow(0_0_12px_color-mix(in_oklab,var(--color-warcraft-danger)_55%,transparent))]",
    ],
    mobile: tw![
        "mobile:border-[length:var(--slot-frame-border,8px)]",
        "mobile:px-[var(--slot-frame-pad-x,var(--spacing))]",
        "mobile:py-[var(--slot-frame-pad-y,calc(var(--spacing)*2))]",
        "mobile:gap-[var(--slot-frame-gap,var(--spacing))]",
    ],
    tablet: tw![
        "tablet:border-[length:var(--slot-frame-border,8px)]",
        "tablet:px-[var(--slot-frame-pad-x,var(--spacing))]",
        "tablet:py-[var(--slot-frame-pad-y,calc(var(--spacing)*2))]",
        "tablet:gap-[var(--slot-frame-gap,var(--spacing))]",
    ],
}
