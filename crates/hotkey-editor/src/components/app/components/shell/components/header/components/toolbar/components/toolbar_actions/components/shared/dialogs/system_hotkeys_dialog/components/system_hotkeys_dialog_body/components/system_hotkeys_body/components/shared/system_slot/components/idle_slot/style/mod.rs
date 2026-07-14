use tw_macro::tw;

// The framed WC3 slot cell: the gold border-image frame, the caption/key layout, and
// the glow filters. It fills the box its host hands it (`size-full`) and draws the
// frame at that size; the host owns the cell's size (aspect ratio, min-height) and
// interaction. The frame's density (border width, padding, gap, image slice) is
// inherited from the parent size container through the `--slot-frame-*` custom
// properties (the control-group row tightens them); with no container override the
// per-band fallbacks reproduce the regular hero/inventory frame. The hover and
// keyboard-focus glows are keyed off the host's named `group/tooltip`. The idle look
// carries no extra glow overlay.
classes! {
    base: tw![
        "size-full",
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-(--slot-frame-gap,--spacing(2))",
        "px-(--slot-frame-pad-x,--spacing(2.5))",
        "py-(--slot-frame-pad-y,--spacing(3.5))",
        "text-center",
        "select-none",
        "border-solid",
        "border-(length:--slot-frame-border,12px)",
        "bg-panel-dark",
        "[border-image-source:var(--wc3-slot-frame)]",
        "[border-image-slice:var(--slot-frame-slice,12_fill)]",
        "[border-image-repeat:stretch]",
        "transition-[filter]",
        "group-hover/tooltip:brightness-[1.18]",
        "group-hover/tooltip:drop-glow",
        "group-focus-visible/tooltip:brightness-[1.25]",
        "group-focus-visible/tooltip:[--glow-color:var(--color-warcraft-highlight)]",
        "group-focus-visible/tooltip:drop-glow",
    ],
    mobile: tw![
        "mobile:border-(length:--slot-frame-border,8px)",
        "mobile:px-(--slot-frame-pad-x,var(--spacing))",
        "mobile:py-(--slot-frame-pad-y,--spacing(2))",
        "mobile:gap-(--slot-frame-gap,var(--spacing))",
    ],
    tablet: tw![
        "tablet:border-(length:--slot-frame-border,8px)",
        "tablet:px-(--slot-frame-pad-x,var(--spacing))",
        "tablet:py-(--slot-frame-pad-y,--spacing(2))",
        "tablet:gap-(--slot-frame-gap,var(--spacing))",
    ],
}
