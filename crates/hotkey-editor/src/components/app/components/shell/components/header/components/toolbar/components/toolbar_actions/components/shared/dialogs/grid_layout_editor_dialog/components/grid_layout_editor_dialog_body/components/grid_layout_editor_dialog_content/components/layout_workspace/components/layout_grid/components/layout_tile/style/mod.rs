use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "group/editable-keycap",
        "flex",
        "items-center",
        "justify-center",
        "size-full",
        "p-0",
        "text-5xl",
        "focus:outline-none",
        "kb-focus:outline-none",
    ],
    mobile: tw![
        "mobile:aspect-square",
        "mobile:text-2xl",
    ],
    tablet: tw![
        "tablet:aspect-square",
        "tablet:text-[10cqi]",
    ],
    laptop: tw![
        "laptop:aspect-square",
        "laptop:text-[10cqi]",
    ],
    desktop: tw![
        "desktop:aspect-square",
        "desktop:text-[10cqi]",
    ],
    qhd: tw![
        "qhd:aspect-square",
        "qhd:text-[10cqi]",
    ],
    uhd: tw![
        "uhd:aspect-square",
        "uhd:text-[10cqi]",
    ],
}
