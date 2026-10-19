use tw_macro::tw;

classes! {
    base: tw![
        "absolute",
        "top-0.5",
        "left-0.5",
        "w-4.5",
        "h-4.5",
        "rounded-full",
        "bg-warcraft-text-muted",
        "transition-[left,background]",
        "duration-base",
        "group-aria-pressed:left-5",
        "group-aria-pressed:bg-warcraft-gold",
    ],
    desktop: tw![
        "desktop:top-0.5",
        "desktop:left-0.5",
        "desktop:w-5.5",
        "desktop:h-5.5",
        "desktop:group-aria-pressed:left-6",
    ],
    qhd: tw![
        "qhd:top-0.75",
        "qhd:left-0.75",
        "qhd:w-7.75",
        "qhd:h-7.75",
        "qhd:group-aria-pressed:left-8.75",
    ],
    uhd: tw![
        "uhd:top-1",
        "uhd:left-1",
        "uhd:w-8.5",
        "uhd:h-8.5",
        "uhd:group-aria-pressed:left-9.25",
    ],
}
