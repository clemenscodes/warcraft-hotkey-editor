use tw_macro::tw;
classes! {
    base: tw![
        "relative",
        "flex",
        "items-center",
        "justify-center",
        "gap-6",
        "flex-none",
        "pt-[1.6rem]",
        "px-[4.5rem]",
        "pb-[1.4rem]",
        "border-b",
        "border-warcraft-gold/40",
        "shadow-edge",
    ],
    mobile: tw![
        "mobile:gap-2",
        "mobile:px-[1.4rem]",
    ],
    tablet: tw![
        "tablet:gap-2",
        "tablet:px-[2rem]",
    ],
}
