use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "relative",
        "flex",
        "items-center",
        "justify-center",
        "gap-6",
        "flex-none",
        "pt-6",
        "px-18",
        "pb-6",
        "border-b",
        "border-warcraft-gold/40",
    ],
    mobile: tw![
        "mobile:gap-2",
        // The close button is absolute at right-2 with w-10, so it owns the
        // rightmost 48px. Padding is what keeps the title and its ornament out
        // of that strip, and it has to stay symmetric to keep the title centred.
        "mobile:px-14",
    ],
    tablet: tw![
        "tablet:gap-2",
        "tablet:px-14",
    ],
}
