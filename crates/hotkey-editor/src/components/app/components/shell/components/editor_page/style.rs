use tw_macro::tw;
// The editor page owns the vertical rhythm between its two sections — the mode/race
// tab bar and the workspace — as a `gap` (tighter on touch, wider from laptop up),
// so neither child reaches across the boundary with a margin. It fills the shell's
// routed slot and lets the workspace grow into the space beneath the tab bar.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "flex-[1_1_0]",
        "min-h-0",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:flex-none",
    ],
    tablet: tw![
        "tablet:flex-none",
    ],
    laptop: tw![
        "laptop:gap-6",
    ],
    desktop: tw![
        "desktop:gap-6",
    ],
    qhd: tw![
        "qhd:gap-6",
    ],
    uhd: tw![
        "uhd:gap-6",
    ],
}
