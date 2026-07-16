use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-center",
        "gap-3.5",
        "pt-4",
        "h-9.5",
    ],
    mobile: tw![
        "mobile:mt-auto",
        "mobile:h-[2.2em]",
        "mobile:gap-[0.6em]",
        "mobile:pt-[0.5em]",
    ],
    tablet: tw![
        "tablet:h-8.5",
    ],
}
