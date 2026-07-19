use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-[1_1_0]",
        "justify-center",
        "min-w-0",
    ],
    // Stacked on a phone, each grid is capped to a readable size and centred,
    // so the before and after grids plus the arrow all fit the card height
    // without overflowing. The cap never exceeds the card width on tiny screens.
    mobile: tw![
        "mobile:flex-none",
        "mobile:w-60",
        "mobile:max-w-full",
    ],
}
