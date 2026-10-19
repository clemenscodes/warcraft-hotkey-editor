use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "w-full",
        "px-6",
        "py-5",
        "rounded-tile",
        "border",
        "border-warcraft-gold/45",
        "bg-warcraft-gold-dark/45",
        "text-center",
        "uppercase",
        "tracking-caps",
        "text-xl",
        "leading-prose",
        "text-warcraft-gold/85",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:normal-case",
        "mobile:tracking-normal",
    ],
    tablet: tw![
        "tablet:text-md",
    ],
    laptop: tw![
        "laptop:text-lg",
    ],
    desktop: tw![
        "desktop:text-lg",
    ],
    qhd: tw![
        "qhd:px-8.5",
        "qhd:py-7",
        "qhd:text-2xl",
    ],
    uhd: tw![
        "uhd:px-10.25",
        "uhd:py-8.5",
        "uhd:text-3xl",
    ],
}
