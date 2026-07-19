use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-1",
        "py-3",
        "text-warcraft-text-secondary",
        "text-lg",
        "leading-prose",
    ],
    mobile: tw![
        "mobile:gap-[0.4em]",
        "mobile:py-[0.5em]",
        "mobile:text-[1em]",
    ],
}
