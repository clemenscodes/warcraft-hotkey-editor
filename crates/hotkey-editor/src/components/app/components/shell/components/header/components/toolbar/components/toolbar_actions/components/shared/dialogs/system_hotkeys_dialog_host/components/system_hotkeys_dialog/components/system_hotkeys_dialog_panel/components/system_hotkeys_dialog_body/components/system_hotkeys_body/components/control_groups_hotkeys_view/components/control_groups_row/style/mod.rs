use tw_macro::tw;

// The ten-cell control-group strip. As the compact size container it publishes the
// `--slot-*` custom properties its slot buttons and framed slots read, tightening the
// frame density, the key glyph size, and the host cell size into the compact
// control-group look — the parent owning the size decision so the slots stay
// size-agnostic.
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(10,11rem)]",
        "gap-3",
        "[--slot-frame-border:8px]",
        "[--slot-frame-slice:12]",
        "[--slot-frame-pad-x:--spacing(1.5)]",
        "[--slot-frame-pad-y:--spacing(3)]",
        "[--slot-frame-gap:--spacing(1.5)]",
        "[--slot-key-size:var(--text-3xl)]",
        "[--slot-host-min-h:--spacing(44)]",
        "[--slot-host-aspect:1]",
    ],
    mobile: tw![
        "mobile:grid-cols-5",
        "mobile:auto-rows-[minmax(72px,auto)]",
        "mobile:gap-1.5",
        "mobile:w-full",
        "mobile:[--slot-frame-border:6px]",
        "mobile:[--slot-frame-pad-x:var(--spacing)]",
        "mobile:[--slot-frame-pad-y:--spacing(1.5)]",
        "mobile:[--slot-frame-gap:var(--spacing)]",
        "mobile:[--slot-key-size:var(--text-base)]",
        "mobile:[--slot-host-min-h:0]",
    ],
    tablet: tw![
        "tablet:grid-cols-5",
        "tablet:auto-rows-[minmax(72px,auto)]",
        "tablet:gap-1.5",
        "tablet:w-full",
        "tablet:[--slot-frame-border:6px]",
        "tablet:[--slot-frame-pad-x:var(--spacing)]",
        "tablet:[--slot-frame-pad-y:--spacing(1.5)]",
        "tablet:[--slot-frame-gap:var(--spacing)]",
        "tablet:[--slot-key-size:var(--text-base)]",
        "tablet:[--slot-host-min-h:0]",
    ],
}
