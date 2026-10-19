use tw_macro::tw;
classes! {
    base: tw![
        "group/editable-keycap",
        "[--keycap-radius:var(--radius-tile)]",
        "flex",
        "items-center",
        "justify-center",
        "w-auto",
        "min-w-20",
        "h-full",
        "p-0",
        "text-xl",
        "whitespace-nowrap",
        "tracking-normal",
        "cursor-pointer",
        "kb-focus:outline-none",
        "kb-focus:shadow-glow-soft",
    ],
    mobile: tw![
        "mobile:min-w-[3.5em]",
        "mobile:text-[1.2em]",
    ],
    tablet: tw![
        "tablet:min-w-[3.5em]",
        "tablet:text-[1.2em]",
    ],
    laptop: tw![
        "laptop:min-w-[3.5em]",
        "laptop:text-[1.2em]",
    ],
    desktop: tw![
        "desktop:min-w-[3.5em]",
        "desktop:text-[1.2em]",
    ],
    qhd: tw![
        "qhd:min-w-[3.5em]",
        "qhd:text-[1.2em]",
    ],
    uhd: tw![
        "uhd:min-w-[3.5em]",
        "uhd:text-[1.2em]",
    ],
}
