use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-end",
        "flex-none",
        "gap-4",
        "pt-[1.4rem]",
        "px-[4.5rem]",
        "pb-[1.8rem]",
        "border-t",
        "border-warcraft-gold/40",
    ],
    mobile: tw![
        "mobile:justify-center",
        "mobile:px-[1.5rem]",
    ],
    tablet: tw!["tablet:justify-center"],
}
