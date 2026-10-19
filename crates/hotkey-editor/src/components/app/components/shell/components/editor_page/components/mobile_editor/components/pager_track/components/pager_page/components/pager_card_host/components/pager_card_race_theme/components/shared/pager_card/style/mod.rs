use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-[4.5cqi]",
        "w-full",
        "flex-1",
        "p-[4.5cqi]",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "bg-panel-dark",
        "shadow-bevel",
    ],
    mobile: tw![
        "mobile:text-[4.02cqi]",
        "mobile:[--grid-heading-size:1em]",
    ],
    tablet: tw![
        "tablet:gap-[2.25cqi]",
        "tablet:p-[2.25cqi]",
        "tablet:text-[1.66cqi]",
        "tablet:[--grid-heading-size:1em]",
    ],
    laptop: tw![
        "laptop:gap-[2.25cqi]",
        "laptop:p-[2.25cqi]",
        "laptop:text-[1.86cqi]",
        "laptop:[--grid-heading-size:1em]",
    ],
}
