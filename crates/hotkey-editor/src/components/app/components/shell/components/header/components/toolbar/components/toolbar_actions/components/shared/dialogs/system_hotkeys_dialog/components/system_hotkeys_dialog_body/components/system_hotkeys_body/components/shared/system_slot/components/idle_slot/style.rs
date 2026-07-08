use tw_macro::tw;

// The framed WC3 slot cell: the gold border-image frame, the caption/key layout,
// and the glow filters. It fills the box its host hands it (`size-full`) and draws
// the frame at that size; the host owns the cell's size (aspect ratio, min-height)
// and interaction. The hover and keyboard-focus glows are keyed off the host's
// named `group/tooltip` (the focusable element), so they react to the host being
// hovered or keyboard-focused while the frame that draws them lives here. `compact`
// tightens the frame for the control-group cell; `dragging` hides the contents
// (caption/key/tooltip) of a slot being dragged while keeping its frame. The idle
// look carries no extra glow overlay.
classes! {
    base: tw![
        "size-full",
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-2",
        "px-2.5",
        "py-3.5",
        "text-center",
        "select-none",
        "border-solid",
        "border-12",
        "bg-panel-dark",
        "[border-image-source:var(--wc3-slot-frame)]",
        "[border-image-slice:12_fill]",
        "[border-image-repeat:stretch]",
        "transition-[filter]",
        "group-hover/tooltip:filter-[brightness(1.18)_drop-shadow(0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent))]",
        "group-focus-visible/tooltip:filter-[brightness(1.25)_drop-shadow(0_0_10px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent))]",
        "data-[dragging=true]:*:invisible",
        "data-[compact=true]:border-8",
        "data-[compact=true]:[border-image-slice:12]",
        "data-[compact=true]:px-1.5",
        "data-[compact=true]:py-3",
        "data-[compact=true]:gap-1.5",
    ],
    mobile: tw![
        "mobile:border-8",
        "mobile:px-1",
        "mobile:py-2",
        "mobile:gap-1",
        "mobile:data-[compact=true]:border-[6px]",
        "mobile:data-[compact=true]:px-1",
        "mobile:data-[compact=true]:py-1.5",
        "mobile:data-[compact=true]:gap-1",
    ],
    tablet: tw![
        "tablet:border-8",
        "tablet:px-1",
        "tablet:py-2",
        "tablet:gap-1",
        "tablet:data-[compact=true]:border-[6px]",
        "tablet:data-[compact=true]:px-1",
        "tablet:data-[compact=true]:py-1.5",
        "tablet:data-[compact=true]:gap-1",
    ],
}
