use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-3",
        "flex-none",
        "m-0",
        "py-5",
        "border-b",
        "border-warcraft-gold/25",
    ],
    mobile: tw![
        "mobile:flex-nowrap",
        "mobile:overflow-x-auto",
        "mobile:overflow-y-hidden",
        "mobile:overscroll-x-contain",
        "mobile:snap-x",
        "mobile:snap-mandatory",
        "mobile:[scrollbar-width:none]",
        "mobile:[&::-webkit-scrollbar]:hidden",
    ],
    tablet: tw![
        "tablet:flex-nowrap",
        "tablet:overflow-x-auto",
        "tablet:overflow-y-hidden",
        "tablet:overscroll-x-contain",
        "tablet:snap-x",
        "tablet:snap-mandatory",
        "tablet:[scrollbar-width:none]",
        "tablet:[&::-webkit-scrollbar]:hidden",
    ],
}
