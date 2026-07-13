use tw_macro::tw;

// The hero-selection / control-group slot host: the focusable, clickable outer
// button. It owns the cell's size (aspect ratio, min-height) and interaction, and is
// the named `group/tooltip` positioning context the framed `SystemSlot` it wraps
// reacts to for its hover/keyboard-focus glows and reveals its tooltip against. Its
// size reads the parent row's `--slot-host-*` custom properties (the control-group
// row tightens them into square, min-height-collapsed cells); with no override the
// per-band fallbacks reproduce the regular hero cell. It draws no frame itself — the
// button chrome is reset so the `SystemSlot` fills it edge to edge.
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
        "min-h-(--slot-host-min-h,auto)",
    ],
    mobile: tw![
        "mobile:aspect-(--slot-host-aspect,1/0.95)",
        "mobile:min-h-(--slot-host-min-h,--spacing(19))",
    ],
    tablet: tw![
        "tablet:aspect-(--slot-host-aspect,1/0.95)",
        "tablet:min-h-(--slot-host-min-h,--spacing(19))",
    ],
}
